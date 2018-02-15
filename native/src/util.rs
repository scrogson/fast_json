use rustler::{Encoder, Env, Term, NifResult};
use atoms;
use errors::*;

pub fn ok<'a>(env: Env<'a>, term: Term<'a>) -> NifResult<Term<'a>> {
    Ok((atoms::ok(), term).encode(env))
}

pub fn error(env: Env, err: Error) -> NifResult<Term> {
    let message = format!("{}", err).encode(env);
    Ok((atoms::error(), message).encode(env))
}
