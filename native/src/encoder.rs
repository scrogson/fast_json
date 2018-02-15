use rustler::{Decoder, Encoder, Env, Term, NifResult, Error};
use rustler::types::atom::Atom;
use rustler::types::list::ListIterator;
use rustler::types::map::MapIterator;
use json;
use json::JsonValue;
use super::util::ok;
use atoms;

pub fn encode<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    let json_val = try!(term_to_json(env, try!(args[0].decode())));
    let json_str = json::stringify(json_val);

    ok(env, json_str.encode(env))
}

fn term_to_json<'a>(env: Env<'a>, term: Term<'a>) -> NifResult<JsonValue> {
    if let Ok(string) = <&str as Decoder>::decode(term) {
        handle_binary(env, string)
    } else if let Ok(iter) = <ListIterator as Decoder>::decode(term) {
        handle_list(env, iter)
    } else if let Ok(atom) = Atom::from_term(term) {
        handle_atom(env, atom)
    } else if let Ok(number) = <f64 as Decoder>::decode(term) {
        handle_float(env, number)
    } else if let Ok(number) = <i64 as Decoder>::decode(term) {
        handle_integer(env, number)
    } else if let Ok(iter) = <MapIterator as Decoder>::decode(term) {
        handle_map(env, iter)
    } else {
        panic!("fail")
    }
}

fn handle_map<'a>(env: Env<'a>, iter: MapIterator<'a>) -> NifResult<JsonValue> {
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
            _ => return Err(Error::BadArg)
        };
        map.insert(&key_string, term_to_json(env, value)?);
    }
    Ok(JsonValue::Object(map))
}

fn handle_list<'a>(env: Env<'a>, iter: ListIterator<'a>) -> NifResult<JsonValue> {
    let values: NifResult<Vec<JsonValue>> = iter.map(|term| {
        term_to_json(env, term)
    }).collect();

    Ok(JsonValue::Array(try!(values)))
}

fn handle_binary<'a>(_env: Env<'a>, string: &str) -> NifResult<JsonValue> {
    Ok(JsonValue::String(string.to_string()))
}

fn handle_atom<'a>(_env: Env<'a>, atom: Atom) -> NifResult<JsonValue> {
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

fn handle_float<'a>(_env: Env<'a>, num: f64) -> NifResult<JsonValue> {
    Ok(JsonValue::Number(num.into()))
}

fn handle_integer<'a>(_env: Env<'a>, num: i64) -> NifResult<JsonValue> {
    Ok(JsonValue::Number(num.into()))
}
