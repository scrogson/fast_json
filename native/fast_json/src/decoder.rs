use std::sync::Mutex;
use rustler::{NifEncoder, NifEnv, NifTerm, NifResult};
use rustler::resource::ResourceCell;
use rustler::schedule::consume_timeslice;
use rustler::tuple::make_tuple;
use rustler::atom::get_atom;
use ::errors::*;
use ::parser::{Parser};
use ::sink::TermSink;

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

fn ok<'a>(env: &'a NifEnv, term: NifTerm) -> NifResult<NifTerm<'a>> {
    let ok = get_atom("ok").unwrap().to_term(env);
    Ok(make_tuple(env, &[ok, term]))
}

fn error<'a>(env: &'a NifEnv, err: Error) -> NifResult<NifTerm<'a>> {
    let error = get_atom("error").unwrap().to_term(env);
    let message = format!("{}", err).encode(env);
    Ok(make_tuple(env, &[error, message]))
}

pub fn decode_init<'a>(env: &'a NifEnv, args: &Vec<NifTerm>) -> NifResult<NifTerm<'a>> {
    let source = args[0].decode()?;
    let mut sink = TermSink::new(env, vec![]);
    let mut parser = Parser::new(source);

    while !consume_timeslice(env, 1) {
        match parser.parse(&mut sink) {
            Ok(true) => return ok(env, sink.pop()),
            Ok(false) => continue,
            Err(err) => return error(env, err)
        }
    }

    // Our timeslice is up.
    let more = get_atom("more").unwrap().to_term(env);
    let parser_resource = ResourceCell::new(ParserResource(Mutex::new(parser)));
    Ok(make_tuple(env, &[more, parser_resource.encode(env), sink.to_stack().encode(env)]))
}
