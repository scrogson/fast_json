use json::{self, JsonValue};
use rustler::env::OwnedEnv;
use rustler::types::map::map_new;
use rustler::{Encoder, Env, Error, Term};
use serde_json::Value;

use crate::atoms;
use crate::POOL;

pub fn decode_naive<'a>(env: Env<'a>, args: &[Term<'a>]) -> Result<Term<'a>, Error> {
    let data = args[0].decode()?;

    match json::parse(data) {
        Ok(json) => {
            let term = json_to_term(env, json);
            Ok((atoms::ok(), term).encode(env))
        }
        Err(err) => {
            let error = format!("{}", err).encode(env);
            Ok((atoms::error(), error).encode(env))
        }
    }
}

pub fn decode_simd<'a>(env: Env<'a>, args: &[Term<'a>]) -> Result<Term<'a>, Error> {
    let mut data: String = args[0].decode()?;

    match simd_json::serde::from_str(&mut data.as_mut_str()) {
        Ok(json) => {
            let term = serde_to_term(env, json);
            Ok((atoms::ok(), term).encode(env))
        }
        Err(err) => {
            let error = format!("{}", err).encode(env);
            Ok((atoms::error(), error).encode(env))
        }
    }
}

pub fn decode_threaded<'a>(caller: Env<'a>, args: &[Term<'a>]) -> Result<Term<'a>, Error> {
    let mut owned_env = OwnedEnv::new();

    let source = owned_env.save(args[0]);
    let caller_pid = caller.pid();

    POOL.spawn(move || {
        owned_env.send_and_clear(&caller_pid, |env| {
            match source.load(env).decode::<String>() {
                Ok(source) => match json::parse(&source) {
                    Ok(json) => {
                        let term = json_to_term(env, json);
                        (atoms::ok(), term).encode(env)
                    }
                    Err(err) => {
                        let error = format!("{}", err).encode(env);
                        (atoms::error(), error).encode(env)
                    }
                },
                Err(_) => atoms::error().encode(env),
            }
        });
    });

    Ok(atoms::ok().encode(caller))
}

fn serde_to_term<'a>(env: Env<'a>, value: Value) -> Term<'a> {
    match value {
        Value::Null => atoms::nil().encode(env),
        Value::String(s) => s.encode(env),
        Value::Number(n) => {
            if n.is_i64() {
                n.as_i64().unwrap().encode(env)
            } else {
                n.as_f64().unwrap().encode(env)
            }
        }
        Value::Bool(b) => b.encode(env),
        Value::Object(mut obj) => obj.iter_mut().fold(map_new(env), |map, (key, value)| {
            let key_term = key.encode(env);
            let value_term = serde_to_term(env, value.take());
            map.map_put(key_term, value_term).ok().unwrap()
        }),
        Value::Array(values) => {
            let terms: Vec<Term<'a>> = values.into_iter().map(|v| serde_to_term(env, v)).collect();
            terms.encode(env)
        }
    }
}

fn json_to_term<'a>(env: Env<'a>, value: JsonValue) -> Term<'a> {
    match value {
        JsonValue::Null => atoms::nil().encode(env),
        JsonValue::Short(s) => s.encode(env),
        JsonValue::String(s) => s.encode(env),
        JsonValue::Number(n) => {
            let (_, _, exponent) = n.as_parts();
            if exponent != 0 {
                f64::from(n).encode(env)
            } else {
                i64::from(n).encode(env)
            }
        }
        JsonValue::Boolean(b) => b.encode(env),
        JsonValue::Object(mut obj) => obj.iter_mut().fold(map_new(env), |map, (key, value)| {
            let key_term = key.encode(env);
            let value_term = json_to_term(env, value.take());
            map.map_put(key_term, value_term).ok().unwrap()
        }),
        JsonValue::Array(values) => {
            let terms: Vec<Term<'a>> = values.into_iter().map(|v| json_to_term(env, v)).collect();
            terms.encode(env)
        }
    }
}
