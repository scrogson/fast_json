use crate::atoms;
use crate::util::ok;
use json::{self, JsonValue};
use rustler::{Atom, Decoder, Encoder, Env, Error, ListIterator, MapIterator, Term};

pub fn encode<'a>(env: Env<'a>, args: &[Term<'a>]) -> Result<Term<'a>, Error> {
    let json_val = term_to_json(env, args[0].decode()?)?;
    let json_str = json::stringify(json_val);

    ok(env, json_str.encode(env))
}

fn term_to_json<'a>(env: Env<'a>, term: Term<'a>) -> Result<JsonValue, Error> {
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

fn handle_map<'a>(env: Env<'a>, iter: MapIterator<'a>) -> Result<JsonValue, Error> {
    use rustler::dynamic::TermType;

    let mut map = json::object::Object::new();

    for (key, value) in iter {
        let key_string = match key.get_type() {
            TermType::Atom => key.atom_to_string().ok().unwrap(),
            TermType::Binary => key.decode().ok().unwrap(),
            _ => return Err(Error::BadArg),
        };
        map.insert(&key_string, term_to_json(env, value)?);
    }
    Ok(JsonValue::Object(map))
}

fn handle_list<'a>(env: Env<'a>, iter: ListIterator<'a>) -> Result<JsonValue, Error> {
    let values: Result<Vec<JsonValue>, _> = iter.map(|term| term_to_json(env, term)).collect();

    Ok(JsonValue::Array(values?))
}

fn handle_binary<'a>(_env: Env<'a>, string: &str) -> Result<JsonValue, Error> {
    Ok(JsonValue::String(string.to_string()))
}

fn handle_atom<'a>(_env: Env<'a>, atom: Atom) -> Result<JsonValue, Error> {
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

fn handle_float<'a>(_env: Env<'a>, num: f64) -> Result<JsonValue, Error> {
    Ok(JsonValue::Number(num.into()))
}

fn handle_integer<'a>(_env: Env<'a>, num: i64) -> Result<JsonValue, Error> {
    Ok(JsonValue::Number(num.into()))
}
