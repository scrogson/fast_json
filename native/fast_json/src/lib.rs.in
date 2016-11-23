extern crate json;

use rustler::{NifDecoder, NifEncoder, NifEnv, NifTerm, NifResult};
use rustler::tuple::*;
use rustler::atom::*;
use rustler::map::*;
use rustler::list::NifListIterator;
use json::Error as JsonError;
use json::JsonValue;
use json::JsonValue::*;

rustler_export_nifs!(
    "Elixir.Json",
    [("native_parse", 2, parse),
     ("stringify", 2, stringify)],
    Some(on_load)
);

macro_rules! handle_parse_error {
    ($env:expr, $err:expr) => {
        return Ok(error_to_term($env, $err))
    };
}

fn error_to_term<'a>(env: &'a NifEnv, err: &JsonError) -> NifTerm<'a> {
    let error = format!("{}", err).encode(env);
    make_tuple(env, &[get_atom("error").unwrap().to_term(env), error])
}

fn on_load(_env: &NifEnv, _info: NifTerm) -> bool {
    init_atom("ok");
    init_atom("error");

    true
}

fn parse<'a>(env: &'a NifEnv, args: &Vec<NifTerm>) -> NifResult<NifTerm<'a>> {
    let data = try!(<&str as NifDecoder>::decode(args[0]));

    match json::parse(data) {
        Ok(val) => {
            let ok = get_atom("ok").unwrap().to_term(env);
            Ok(make_tuple(env, &[ok, json_to_term(env, val)]))
        },
        Err(err) =>  {
            handle_parse_error!(env, &err)
        }
    }
}

fn json_to_term<'a>(env: &'a NifEnv, value: JsonValue) -> NifTerm<'a> {
    match value {
        Null => get_atom("nil").unwrap().to_term(env),
        Short(s) => s.encode(env),
        String(s) => s.encode(env),
        Number(n) => {
            let (_, _, exponent) = n.as_parts();
            if exponent != 0 {
                f64::from(n).encode(env)
            } else {
                i64::from(n).encode(env)
            }
        }
        Boolean(b) => b.encode(env),
        Object(mut obj) => {
            obj.iter_mut().fold(map_new(env), |map, (key, value)| {
                let key_term = key.encode(env);
                let value_term = json_to_term(env, value.take());
                map_put(map, key_term, value_term).unwrap()
            })
        }
        Array(values) => {
            let terms: Vec<NifTerm<'a>> = values.into_iter()
                .map(|v| json_to_term(env, v))
                .collect();
            terms.encode(env)
        }
    }
}

fn stringify<'a>(env: &'a NifEnv, args: &Vec<NifTerm>) -> NifResult<NifTerm<'a>> {
    let json_val = try!(term_to_json(env, try!(args[0].decode())));
    let json_str = json::stringify(json_val);

    Ok(json_str.encode(env))
}

fn term_to_json<'a>(env: &'a NifEnv, term: NifTerm) -> NifResult<JsonValue> {
    if let Ok(string) = <&str as NifDecoder>::decode(term) {
        handle_binary(env, string)
    } else if let Ok(iter) = <NifListIterator as NifDecoder>::decode(term) {
        handle_list(env, iter)
    } else if let Some(atom) = NifAtom::from_term(env, term) {
        handle_atom(env, atom)
    } else if let Ok(number) = <f64 as NifDecoder>::decode(term) {
        handle_float(env, number)
    } else if let Ok(number) = <i64 as NifDecoder>::decode(term) {
        handle_integer(env, number)
    } else {
        panic!("fail")
    }
}

//fn handle_map(env: &NifEnv, iter: NifMapIterator) -> NifResult<JsonValue> {
    //let values: NifResult<Vec<JsonValue>> = iter.map(|term| {
        //term_to_json(env, term)
    //}).collect();

    //Ok(JsonValue::Array(try!(values)))
//}

fn handle_list(env: &NifEnv, iter: NifListIterator) -> NifResult<JsonValue> {
    let values: NifResult<Vec<JsonValue>> = iter.map(|term| {
        term_to_json(env, term)
    }).collect();

    Ok(JsonValue::Array(try!(values)))
}

fn handle_binary(_env: &NifEnv, string: &str) -> NifResult<JsonValue> {
    Ok(JsonValue::String(string.to_string()))
}

fn handle_atom(_env: &NifEnv, atom: NifAtom) -> NifResult<JsonValue> {
    if atom == get_atom("true").unwrap() {
        Ok(JsonValue::Boolean(true))
    } else if atom == get_atom("false").unwrap() {
        Ok(JsonValue::Boolean(false))
    } else if atom == get_atom("nil").unwrap() {
        Ok(JsonValue::Null)
    } else {
        Ok(JsonValue::String("nope".to_string()))
    }
}

fn handle_float(_env: &NifEnv, num: f64) -> NifResult<JsonValue> {
    Ok(JsonValue::Number(num.into()))
}

fn handle_integer(_env: &NifEnv, num: i64) -> NifResult<JsonValue> {
    Ok(JsonValue::Number(num.into()))
}
