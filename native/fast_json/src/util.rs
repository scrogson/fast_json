use rustler::{NifEncoder, NifEnv, NifTerm, NifResult};
use rustler::types::atom::get_atom;
use rustler::types::tuple::make_tuple;
use errors::*;

pub fn ok<'a>(env: NifEnv<'a>, term: NifTerm<'a>) -> NifResult<NifTerm<'a>> {
    let ok = get_atom("ok").unwrap().to_term(env);
    Ok(make_tuple(env, &[ok, term]))
}

pub fn error(env: NifEnv, err: Error) -> NifResult<NifTerm> {
    let error = get_atom("error").unwrap().to_term(env);
    let message = format!("{}", err).encode(env);
    Ok(make_tuple(env, &[error, message]))
}
