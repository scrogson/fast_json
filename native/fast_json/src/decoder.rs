use std::sync::Mutex;

use json::{self, JsonValue};
use rustler::schedule::consume_timeslice;
use rustler::types::map::map_new;
use rustler::{Atom, Encoder, Env, Error, OwnedEnv, ResourceArc, Term};

use crate::atoms;
use crate::parser::Parser;
use crate::sink::TermSink;
use crate::util::{error, ok};
use crate::POOL;

pub struct ParserResource(Mutex<Parser>);

impl ParserResource {
    fn new(data: String) -> ParserResource {
        ParserResource(Mutex::new(Parser::new(data)))
    }
}

#[rustler::nif]
pub fn decode_naive(env: Env, data: String) -> Result<Term, Error> {
    parse_json(env, data)
}

#[rustler::nif(schedule = "DirtyCpu")]
pub fn decode_dirty(env: Env, data: String) -> Result<Term, Error> {
    parse_json(env, data)
}

#[rustler::nif]
pub fn decode_init<'a>(data: String) -> (Atom, ResourceArc<ParserResource>, Vec<Term<'a>>) {
    let resource = ResourceArc::new(ParserResource::new(data));
    let vector: Vec<Term> = vec![];

    (atoms::more(), resource, vector)
}

#[rustler::nif]
pub fn decode_iter<'a>(
    env: Env<'a>,
    resource: ResourceArc<ParserResource>,
    stack: Vec<Term<'a>>,
) -> Result<Term<'a>, Error> {
    let mut sink = TermSink::new(env, stack);
    let mut parser = match resource.0.try_lock() {
        Err(_) => return Err(Error::BadArg),
        Ok(guard) => guard,
    };

    while !consume_timeslice(env, 1) {
        match parser.parse(&mut sink) {
            Ok(true) => return ok(env, sink.pop()),
            Ok(false) => continue,
            Err(err) => return error(env, err),
        }
    }

    Ok((atoms::more(), resource.clone(), sink.to_stack()).encode(env))
}

#[rustler::nif]
pub fn decode_threaded(env: Env, data: Term) -> Atom {
    let mut owned_env = OwnedEnv::new();

    let source = owned_env.save(data);
    let caller_pid = env.pid();

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

    atoms::ok()
}

fn parse_json(env: Env, data: String) -> Result<Term, Error> {
    match json::parse(&data) {
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
