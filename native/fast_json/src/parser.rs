use rustler::{NifTerm, NifEnv, NifEncoder};
use rustler::map::{map_new, map_put};
use rustler::atom::init_atom;

use error_chain;

pub mod errors {
    error_chain! {
        errors {
            InvalidJson(message: &'static str, offset: usize) {
                description(message)
                display("{} (at offset {})", message, offset)
            }
        }
    }
}

use self::errors::*;

/// Answers the question "when I'm done parsing this value, where does it go?"
enum Target<'a> {
    IntoArray {
        elements: Vec<NifTerm<'a>>,
    },
    IntoObject {
        map: NifTerm<'a>,
        key: Option<NifTerm<'a>>
    }
}

struct Parser<'a> {
    /// Reference to the Erlang virtual machine.
    env: &'a NifEnv,

    /// Source string that we're parsing.
    s: String,

    /// Current read position within the string.
    i: usize,

    /// Stack of still-open objects and arrays.
    stack: Vec<Target<'a>>
}

fn is_whitespace(value: u8) -> bool {
   match value {
       b'\t' | b'\r' | b'\n' | b' ' => true,
       _ => false,
   }
}

const BACKSPACE: char = 8 as char;
const FORM_FEED: char = 12 as char;

impl<'a> Parser<'a> {
    fn new(env: &'a NifEnv, s: String) -> Parser<'a> {
        Parser {
            env: env,
            s: s,
            i: 0,
            stack: vec![]
        }
    }

    fn skip_ws(&mut self) {
        while !self.at_end() && is_whitespace(self.peek_next_byte()) {
            self.i += 1;
        }
    }

    fn parse_string(&mut self) -> Result<NifTerm<'a>> {
        assert_eq!(self.peek_next_byte(), b'"');
        let start = self.i;
        self.i += 1;
        let mut strval = String::new();

        let mut iter = self.s[start..].char_indices();
        while let Some((j, c)) = iter.next() {
            if c == '"' {
                strval += &self.s[self.i .. start + j];
                self.i = start + j;
                self.i += 1;  // also skip the quote mark itself
                return Ok(strval.encode(self.env));
            } else if c == '\\' {
                strval += &self.s[self.i .. start + j];
                self.i = start + j;
                self.i += 1;  // also skip the backslash itself
                match iter.next() {
                    None => break,
                    Some((_, esc_char)) => {
                        let out_char = match esc_char {
                            '"' => '"',
                            '\\' => '\\',
                            '/' => '/',
                            'b' => BACKSPACE,
                            'f' => FORM_FEED,
                            'n' => '\n',
                            'r' => '\r',
                            't' => '\t',
                            'u' => panic!("oh no not supported yet"),
//                 elif c == 'u':
//                     if self.i + 4 >= len(self.s):
//                         break
//                     hexcode = self.s[self.i:self.i + 4]
//                     for c in hexcode:
//                         if c not in '0123456789abcdefABCDEF':
//                             self.fail("invalid \\u escape")
//                     self.i += 4
//                     strval += chr(int(hexcode, 16))
                            _ => return Err(self.fail("invalid character escape"))
                        };
                        self.i += 1;
                        strval.push(out_char);
                    }
                }
            } else if c.is_control() {
                return Err(self.fail("unexpected control character in string"));
            } else {
                // do nothing, we'll copy it into strval later
            }
        }
        Err(self.fail("unterminated string literal"))
    }

    fn fail(&self, message: &'static str) -> Error {
        ErrorKind::InvalidJson(message, self.i).into()
    }

    fn parse_key(&mut self) -> Result<NifTerm<'a>> {
        self.skip_ws();
        if self.at_end() || self.peek_next_byte() != b'"' {
            return Err(self.fail("expected key in object"));
        }
        let key = self.parse_string()?;
        self.skip_ws();
        if self.at_end() || self.peek_next_byte() != b':' {
            return Err(self.fail("expected ':' after key in object"));
        }
        self.i += 1;
        Ok(key)
    }

    fn at_end(&self) -> bool {
        self.i >= self.s.len()
    }

    fn peek_next_byte(&self) -> u8 {
        self.s.as_bytes()[self.i]
    }

    fn parse_one_value(&mut self) -> Result<NifTerm<'a>> {
        loop {
            self.skip_ws();
            if self.at_end() {
                let msg = match self.stack.last() {
                    Some(&Target::IntoArray { .. }) => "unexpected end of input in array",
                    Some(&Target::IntoObject { .. }) => "unexpected end of input in object",
                    None => "input was all whitespace"
                };
                return Err(self.fail(msg));
            }

            let value = match self.peek_next_byte() {
                b'-' | b'0' ... b'9' => {
                    let start = self.i;
                    while !self.at_end() && b"+-0123456789.eE".contains(&self.peek_next_byte()) {
                        self.i += 1;
                    }
                    let numstr = &self.s[start .. self.i];
                    if numstr.contains('.') || numstr.contains('e') || numstr.contains("E") {
                        let number: f64 = numstr.parse().chain_err(|| "invalid number")?;
                        number.encode(self.env)
                    } else {
                        let number: i64 = numstr.parse().chain_err(|| "invalid number")?;
                        number.encode(self.env)
                    }
                }

                b'"' => self.parse_string()?,

                b't' if self.s[self.i..].starts_with("true") => {
                    self.i += 4;
                    true.encode(self.env)
                }
                b'f' if self.s[self.i..].starts_with("false") => {
                    self.i += 5;
                    false.encode(self.env)
                }
                b'n' if self.s[self.i..].starts_with("null") => {
                    self.i += 4;
                    init_atom("nil").to_term(self.env)
                }

                b'{' => {
                    self.i += 1;
                    self.skip_ws();
                    if !self.at_end() && self.peek_next_byte() == b'}' {
                        self.i += 1;
                        map_new(self.env)
                    } else {
                        let key = self.parse_key()?;
                        self.stack.push(Target::IntoObject {
                            map: map_new(self.env),
                            key: Some(key)
                        });
                        continue;
                    }
                }

                b'}' => {
                    self.i += 1;
                    match self.stack.pop() {
                        Some(Target::IntoObject { map, key }) => {
                            assert!(key.is_none());
                            map
                        }
                        _ => return Err(self.fail("found '}' without matching '{'"))
                    }
                }

                b'[' => {
                    self.i += 1;
                    self.stack.push(Target::IntoArray {
                        elements: vec![]
                    });
                    continue;
                }

                b']' => {
                    self.i += 1;
                    match self.stack.pop() {
                        Some(Target::IntoArray { elements }) =>
                            elements.encode(self.env),
                        _ =>
                            return Err(self.fail("found ']' without matching '['"))
                    }
                }

                _ => return Err(self.fail("unexpected character"))
            };
            return Ok(value);
        }
    }

    fn store_value(&mut self, v: NifTerm<'a>) -> Result<()> {
        match self.stack.pop() {
            Some(Target::IntoObject { map, key }) => {
                let new_map = map_put(map, key.unwrap(), v).unwrap();
                self.skip_ws();
                if self.at_end() {
                    return Err(self.fail("unmatched '{'"));
                }
                match self.peek_next_byte() {
                    b',' => {
                        self.i += 1;
                        let new_key = self.parse_key()?;
                        self.stack.push(Target::IntoObject {
                            map: new_map,
                            key: Some(new_key)
                        });
                    }
                    b'}' => {
                        self.stack.push(Target::IntoObject {
                            map: new_map,
                            key: None
                        });
                    }
                    _ => {
                        return Err(self.fail("expected ',' or '}' after key-value pair in object"));
                    }
                }
            }
            Some(Target::IntoArray { mut elements }) => {
                elements.push(v);
                self.skip_ws();
                if self.at_end() {
                    return Err(self.fail("unmatched '['"));
                }
                match self.peek_next_byte() {
                    b',' => self.i += 1,
                    b']' => {}
                    _ => return Err(self.fail("expected ',' or ']' after array element"))
                }
                self.stack.push(Target::IntoArray { elements: elements });
            }
            None => panic!("can't happen")
        }
        Ok(())
    }

    fn finish(&mut self) -> Result<()> {
        self.skip_ws();
        if self.i < self.s.len() {
            return Err(self.fail("unexpected extra data after JSON"));
        }
        Ok(())
    }

    fn parse_some(&mut self, limit: usize) -> Result<Option<NifTerm<'a>>> {
        for _ in 0 .. limit {
            let v = self.parse_one_value()?;
            if self.stack.is_empty() {
                self.finish()?;
                return Ok(Some(v));
            } else {
                self.store_value(v)?;
            }
        }
        Ok(None)
    }
}


pub fn parse<'a>(env: &'a NifEnv, s: String) -> Result<NifTerm<'a>> {
    let mut p = Parser::new(env, s);
    loop {
        match p.parse_some(1)? {
            None => {}
            Some(value) => return Ok(value)
        }
    }
}
