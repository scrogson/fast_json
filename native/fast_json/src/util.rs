use rustler::{NifEncoder, NifEnv, NifTerm, NifResult};
use rustler::types::atom::get_atom;
use rustler::types::tuple::make_tuple;
use errors::*;

pub fn ok<'a>(env: &'a NifEnv, term: NifTerm) -> NifResult<NifTerm<'a>> {
    let ok = get_atom("ok").unwrap().to_term(env);
    Ok(make_tuple(env, &[ok, term]))
}

pub fn error<'a>(env: &'a NifEnv, err: Error) -> NifResult<NifTerm<'a>> {
    let error = get_atom("error").unwrap().to_term(env);
    let message = format!("{}", err).encode(env);
    Ok(make_tuple(env, &[error, message]))
}
