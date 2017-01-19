use json::{self, JsonValue};
use rustler::types::map::map_new;

use std::sync::Mutex;
use rustler::{NifEncoder, NifEnv, NifTerm, NifResult, NifError};
use rustler::resource::ResourceCell;
use rustler::schedule::consume_timeslice;
use rustler::thread;
use parser::Parser;
use sink::TermSink;
use util::{ok, error};
use atoms;

pub struct ParserResource(Mutex<Parser>);

pub fn decode_naive<'a>(env: NifEnv<'a>, args: &Vec<NifTerm<'a>>) -> NifResult<NifTerm<'a>> {
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

pub fn decode_init<'a>(env: NifEnv<'a>, args: &Vec<NifTerm<'a>>) -> NifResult<NifTerm<'a>> {
    let source = args[0].decode()?;
    let resource = ResourceCell::new(ParserResource(Mutex::new(Parser::new(source))));
    let vector: Vec<NifTerm<'a>> = vec![];

    decode_iter(env, &vec![resource.encode(env), vector.encode(env)])
}

pub fn decode_iter<'a>(env: NifEnv<'a>, args: &Vec<NifTerm<'a>>) -> NifResult<NifTerm<'a>> {
    let resource: ResourceCell<ParserResource> = args[0].decode()?;
    let sink_stack: Vec<NifTerm> = args[1].decode()?;

    let mut sink = TermSink::new(env, sink_stack);
    let mut parser = match resource.0.try_lock() {
        Err(_) => return Err(NifError::BadArg),
        Ok(guard) => guard,
    };

    while !consume_timeslice(env, 1) {
        match parser.parse(&mut sink) {
            Ok(true) => return ok(env, sink.pop()),
            Ok(false) => continue,
            Err(err) => return error(env, err),
        }
    }

    Ok((atoms::more(), args[0], sink.to_stack()).encode(env))
}

pub fn decode_threaded<'a>(caller: NifEnv<'a>, args: &Vec<NifTerm<'a>>) -> NifResult<NifTerm<'a>> {
    let source: String = args[0].decode()?;

    thread::spawn::<thread::ThreadSpawner, _>(caller, move |env| {
        match json::parse(&source) {
            Ok(json) => {
                let term = json_to_term(env, json);
                (atoms::ok(), term).encode(env)
            }
            Err(err) => {
                let error = format!("{}", err).encode(env);
                (atoms::error(), error).encode(env)
            }
        }
    });
    Ok(atoms::ok().to_term(caller))
}

fn json_to_term<'a>(env: NifEnv<'a>, value: JsonValue) -> NifTerm<'a> {
    match value {
        JsonValue::Null => atoms::nil().to_term(env),
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
        JsonValue::Object(mut obj) => {
            obj.iter_mut().fold(map_new(env), |map, (key, value)| {
                let key_term = key.encode(env);
                let value_term = json_to_term(env, value.take());
                map.map_put(key_term, value_term).ok().unwrap()
            })
        }
        JsonValue::Array(values) => {
            let terms: Vec<NifTerm<'a>> = values.into_iter()
                .map(|v| json_to_term(env, v))
                .collect();
            terms.encode(env)
        }
    }
}
