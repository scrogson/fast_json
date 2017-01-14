use rustler::{NifEncoder, NifEnv, NifTerm, NifResult};
use atoms;
use errors::*;

pub fn ok<'a>(env: NifEnv<'a>, term: NifTerm<'a>) -> NifResult<NifTerm<'a>> {
    Ok((atoms::ok(), term).encode(env))
}

pub fn error(env: NifEnv, err: Error) -> NifResult<NifTerm> {
    let message = format!("{}", err).encode(env);
    Ok((atoms::error(), message).encode(env))
}
