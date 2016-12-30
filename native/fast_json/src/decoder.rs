use rustler::{NifDecoder, NifEncoder, NifEnv, NifTerm, NifResult};
use rustler::tuple::*;
use rustler::atom::*;
use rustler::map::*;
use json;
use json::Error as JsonError;
use json::JsonValue;
use json::JsonValue::*;

macro_rules! handle_parse_error {
    ($env:expr, $err:expr) => {
        return Ok(error_to_term($env, $err))
    };
}

fn error_to_term<'a>(env: &'a NifEnv, err: &JsonError) -> NifTerm<'a> {
    let error = format!("{}", err).encode(env);
    make_tuple(env, &[get_atom("error").unwrap().to_term(env), error])
}

pub fn decode<'a>(env: &'a NifEnv, args: &Vec<NifTerm>) -> NifResult<NifTerm<'a>> {
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
