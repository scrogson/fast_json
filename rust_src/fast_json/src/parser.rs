use ::sink::{TermSink, ValueSink};
use ::errors::*;

const BACKSPACE: char = 8 as char;
const FORM_FEED: char = 12 as char;

#[derive(Debug)]
enum Stack {
    Array,
    Object {
        key: Option<String>,
    }
}

pub struct Parser {
    /// Source string that we're parsing.
    s: String,

    /// Current read position within the string.
    i: usize,

    /// Stack of still-open objects and arrays.
    stack: Vec<Stack>,
}

fn is_whitespace(value: u8) -> bool {
   match value {
       b'\t' | b'\r' | b'\n' | b' ' => true,
       _ => false,
   }
}

impl Parser {
    pub fn new(s: String) -> Parser {
        Parser {
            s: s,
            i: 0,
            stack: vec![]
        }
    }

    fn push(&mut self, value: Stack) {
        self.stack.push(value);
    }

    fn pop(&mut self) -> Option<Stack> {
        self.stack.pop()
    }

    fn skip_ws(&mut self) {
        while !self.at_end() && is_whitespace(self.peek_next_byte()) {
            self.i += 1;
        }
    }

    fn parse_string(&mut self) -> Result<String> {
        assert_eq!(self.peek_next_byte(), b'"');
        self.i += 1;
        let start = self.i;
        let mut strval = String::new();

        let mut iter = self.s[start..].char_indices();
        while let Some((j, c)) = iter.next() {
            if c == '"' {
                strval += &self.s[self.i .. start + j];
                self.i = start + j;
                self.i += 1;  // also skip the quote mark itself
                return Ok(strval);
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
                            token => return Err(self.fail_string(format!("Unexpected token {} in JSON", token)))
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
        Err(self.fail("Invalid or unexpected token"))
    }

    fn fail(&self, message: &'static str) -> Error {
        self.fail_string(message.to_string())
    }

    fn fail_string(&self, message: String) -> Error {
        ErrorKind::InvalidJson(message.to_string(), self.i).into()
    }

    fn parse_key(&mut self) -> Result<String> {
        self.skip_ws();
        if self.at_end() || self.peek_next_byte() != b'"' {
            return Err(self.fail("Unexpected end of JSON input"));
        }
        let key = self.parse_string()?;
        self.skip_ws();
        if self.at_end() || self.peek_next_byte() != b':' {
            return Err(self.fail_string(format!("Unexpected token {} in JSON", self.peek_next_byte() as char)));
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

    fn parse_one_value(&mut self, sink: &mut TermSink) -> Result<()> {
        loop {
            self.skip_ws();
            if self.at_end() {
                let msg = match self.stack.last() {
                    _ => "Unexpected end of JSON input"
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
                        let number: f64 = numstr.parse().chain_err(|| self.fail("Unexpected number in JSON"))?;
                        sink.push_float(number);
                    } else {
                        let number: i64 = numstr.parse().chain_err(|| self.fail("Unexpected number in JSON"))?;
                        sink.push_integer(number);
                    }
                }

                b'"' => {
                    sink.push_string(self.parse_string()?);
                },

                b't' if self.s[self.i..].starts_with("true") => {
                    self.i += 4;
                    sink.push_bool(true);
                }
                b'f' if self.s[self.i..].starts_with("false") => {
                    self.i += 5;
                    sink.push_bool(false);
                }
                b'n' if self.s[self.i..].starts_with("null") => {
                    self.i += 4;
                    sink.push_nil();
                }

                b'{' => {
                    self.i += 1;
                    self.skip_ws();
                    if !self.at_end() && self.peek_next_byte() == b'}' {
                        self.i += 1;
                        sink.push_map();
                    } else {
                        let key = self.parse_key()?;
                        sink.push_map(); // should not call pop_insert_*
                        self.push(Stack::Object { key: Some(key) });
                        continue;
                    }
                }

                b'}' => {
                    self.i += 1;
                    match self.pop() {
                        Some(Stack::Object { key }) => {
                            assert!(key.is_none());
                            sink.finalize_map();
                        }
                        _ => return Err(self.fail("found '}' without matching '{'"))
                    }
                }

                b'[' => {
                    self.i += 1;
                    sink.push_array();
                    self.push(Stack::Array);
                    continue;
                }

                b']' => {
                    self.i += 1;
                    match self.pop() {
                        Some(Stack::Array) =>
                            sink.finalize_array(),
                        _ =>
                            return Err(self.fail("found ']' without matching '['"))
                    }
                }

                token => return Err(self.fail_string(format!("Unexpected token {}", token as char)))
            };
            return Ok(value);
        }
    }

    fn store_value(&mut self, sink: &mut TermSink) -> Result<()> {
        match self.stack.pop() {
            Some(Stack::Object { key }) => {
                sink.pop_insert_map(key.unwrap());
                self.skip_ws();
                if self.at_end() {
                    return Err(self.fail("unmatched '{'"));
                }
                match self.peek_next_byte() {
                    b',' => {
                        self.i += 1;
                        let new_key = self.parse_key()?;
                        self.stack.push(Stack::Object { key: Some(new_key) });
                    }
                    b'}' => {
                        self.stack.push(Stack::Object { key: None });
                    }
                    _ => {
                        return Err(self.fail("expected ',' or '}' after key-value pair in object"));
                    }
                }
            }
            Some(Stack::Array) => {
                sink.pop_insert_array();
                self.skip_ws();
                if self.at_end() {
                    return Err(self.fail("unmatched '['"));
                }
                match self.peek_next_byte() {
                    b',' => self.i += 1,
                    b']' => {}
                    _ => return Err(self.fail("expected ',' or ']' after array element"))
                }
                self.stack.push(Stack::Array);
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

    pub fn parse(&mut self, sink: &mut TermSink) -> Result<bool> {
        self.parse_one_value(sink)?;

        if self.stack.is_empty() {
            self.finish()?;
            Ok(true)
        } else {
            self.store_value(sink)?;
            Ok(false)
        }
    }
}
