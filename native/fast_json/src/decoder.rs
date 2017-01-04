use std::sync::Mutex;
use rustler::{NifEncoder, NifEnv, NifTerm, NifResult, NifError};
use rustler::resource::ResourceCell;
use rustler::schedule::consume_timeslice;
use rustler::tuple::make_tuple;
use rustler::atom::get_atom;
use errors::*;
use parser::{Parser};
use sink::TermSink;
use threaded_awesomeness;

pub struct ParserResource(Mutex<Parser>);

pub fn naive_parse<'a>(env: &'a NifEnv, args: &Vec<NifTerm>) -> NifResult<NifTerm<'a>> {
    let source = args[0].decode()?;

    match naive(env, source) {
        Ok(term) => ok(env, term),
        Err(err) => error(env, err)
    }
}

pub fn naive<'a>(env: &'a NifEnv, source: String) -> Result<NifTerm<'a>> {
    let mut sink = TermSink::new(env, vec![]);
    let mut parser = Parser::new(source);

    loop {
        if parser.parse(&mut sink)? {
            return Ok(sink.pop());
        }
    }
}

pub fn decode_init<'a>(env: &'a NifEnv, args: &Vec<NifTerm>) -> NifResult<NifTerm<'a>> {
    let source = args[0].decode()?;
    let resource = ResourceCell::new(ParserResource(Mutex::new(Parser::new(source))));
    let vector: Vec<NifTerm<'a>> = vec![];

    decode_iter(env, &vec![resource.encode(env), vector.encode(env)])
}

pub fn decode_iter<'a>(env: &'a NifEnv, args: &Vec<NifTerm>) -> NifResult<NifTerm<'a>> {
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
            Err(err) => return error(env, err)
        }
    }

    let more = get_atom("more").unwrap().to_term(env);
    Ok(make_tuple(env, &[more, args[0], sink.to_stack().encode(env)]))
}

pub fn decode_threaded<'a>(env: &'a NifEnv, args: &Vec<NifTerm>) -> NifResult<NifTerm<'a>> {
    let source = args[0].decode()?;
    let mut parser = Parser::new(source);

    threaded_awesomeness::spawn_thread_and_send_value_back_to_self(
        env,
        move |env| {
            let mut sink = TermSink::new(env, vec![]);
            loop {
                match parser.parse(&mut sink) {
                    Ok(true) => return ok(env, sink.pop()).ok().unwrap(),
                    Ok(false) => continue,
                    Err(err) => return error(env, err).ok().unwrap()
                }
            }
        });
    Ok(get_atom("ok").unwrap().to_term(env))
}

pub fn ok<'a>(env: &'a NifEnv, term: NifTerm) -> NifResult<NifTerm<'a>> {
    let ok = get_atom("ok").unwrap().to_term(env);
    Ok(make_tuple(env, &[ok, term]))
}

pub fn error<'a>(env: &'a NifEnv, err: Error) -> NifResult<NifTerm<'a>> {
    let error = get_atom("error").unwrap().to_term(env);
    let message = format!("{}", err).encode(env);
    Ok(make_tuple(env, &[error, message]))
}

