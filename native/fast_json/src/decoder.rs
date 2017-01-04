use rustler::{NifEncoder, NifEnv, NifTerm, NifResult};
use rustler::tuple::make_tuple;
use rustler::atom::get_atom;
use ::errors::*;
use ::parser;

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

    match parser::naive(env, source) {
        Ok(term) => ok(env, term),
        Err(err) => error(env, err)
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
