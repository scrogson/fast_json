use rustler::{NifEncoder, NifEnv, NifTerm, NifResult};
use rustler::tuple::make_tuple;
use rustler::atom::get_atom;
use ::errors::*;
use ::parser::{Parser};
use ::sink::TermSink;

// chucking...
// consume_timeslice...
// pub fn chuck_parse(...)
// chuck_parse(binary)
// initialize the resource
//
// chuck_parse_iter
// decode the resource and make a parser from it
// decode sink stack and make a new TermSink::new(env, stack)
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
        //if consume_timeslice(env, 1) {
            //{:iter, resource, sink.stack.encode(sink.env)}
        //}

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
