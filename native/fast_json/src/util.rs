use crate::atoms;
use crate::errors;
use rustler::{Encoder, Env, Error, Term};

pub fn ok<'a>(env: Env<'a>, term: Term<'a>) -> Result<Term<'a>, Error> {
    Ok((atoms::ok(), term).encode(env))
}

pub fn error(env: Env, err: errors::Error) -> Result<Term, Error> {
    let message = format!("{}", err).encode(env);
    Ok((atoms::error(), message).encode(env))
}
