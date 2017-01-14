use rustler::{NifDecoder, NifEncoder, NifEnv, NifTerm, NifResult, NifError};
use rustler::types::atom::NifAtom;
use rustler::types::list::NifListIterator;
use rustler::types::map::NifMapIterator;
use json;
use json::JsonValue;
use super::util::ok;
use atoms;

pub fn encode<'a>(env: NifEnv<'a>, args: &Vec<NifTerm<'a>>) -> NifResult<NifTerm<'a>> {
    let json_val = try!(term_to_json(env, try!(args[0].decode())));
    let json_str = json::stringify(json_val);

    ok(env, json_str.encode(env))
}

fn term_to_json<'a>(env: NifEnv<'a>, term: NifTerm<'a>) -> NifResult<JsonValue> {
    if let Ok(string) = <&str as NifDecoder>::decode(term) {
        handle_binary(env, string)
    } else if let Ok(iter) = <NifListIterator as NifDecoder>::decode(term) {
        handle_list(env, iter)
    } else if let Ok(atom) = NifAtom::from_term(term) {
        handle_atom(env, atom)
    } else if let Ok(number) = <f64 as NifDecoder>::decode(term) {
        handle_float(env, number)
    } else if let Ok(number) = <i64 as NifDecoder>::decode(term) {
        handle_integer(env, number)
    } else if let Ok(iter) = <NifMapIterator as NifDecoder>::decode(term) {
        handle_map(env, iter)
    } else {
        panic!("fail")
    }
}

fn handle_map<'a>(env: NifEnv<'a>, iter: NifMapIterator<'a>) -> NifResult<JsonValue> {
    use rustler::dynamic::TermType;

    let mut map = json::object::Object::new();

    for (key, value) in iter {
        let key_string = match key.get_type() {
            TermType::Atom => {
                key.atom_to_string().ok().unwrap()
            }
            TermType::Binary => {
                key.decode().ok().unwrap()
            }
            _ => return Err(NifError::BadArg)
        };
        map.insert(&key_string, term_to_json(env, value)?);
    }
    Ok(JsonValue::Object(map))
}

fn handle_list<'a>(env: NifEnv<'a>, iter: NifListIterator<'a>) -> NifResult<JsonValue> {
    let values: NifResult<Vec<JsonValue>> = iter.map(|term| {
        term_to_json(env, term)
    }).collect();

    Ok(JsonValue::Array(try!(values)))
}

fn handle_binary<'a>(_env: NifEnv<'a>, string: &str) -> NifResult<JsonValue> {
    Ok(JsonValue::String(string.to_string()))
}

fn handle_atom<'a>(_env: NifEnv<'a>, atom: NifAtom) -> NifResult<JsonValue> {
    if atom == atoms::__true__() {
        Ok(JsonValue::Boolean(true))
    } else if atom == atoms::__false__() {
        Ok(JsonValue::Boolean(false))
    } else if atom == atoms::nil() {
        Ok(JsonValue::Null)
    } else {
        Ok(JsonValue::String("nope".to_string()))
    }
}

fn handle_float<'a>(_env: NifEnv<'a>, num: f64) -> NifResult<JsonValue> {
    Ok(JsonValue::Number(num.into()))
}

fn handle_integer<'a>(_env: NifEnv<'a>, num: i64) -> NifResult<JsonValue> {
    Ok(JsonValue::Number(num.into()))
}
