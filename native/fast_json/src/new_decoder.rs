use std::error::Error;
use super::parser;
use rustler::{NifDecoder, NifEncoder, NifEnv, NifTerm, NifResult};
use rustler::tuple::make_tuple;
use rustler::atom::get_atom;

pub fn new_decode<'a>(env: &'a NifEnv, args: &Vec<NifTerm>) -> NifResult<NifTerm<'a>> {
    let data = <String as NifDecoder>::decode(args[0])?;

    match parser::parse(env, data) {
        Ok(term) => {
            let ok = get_atom("ok").unwrap().to_term(env);
            Ok(make_tuple(env, &[ok, term]))
        }
        Err(err) =>  {
            let error = get_atom("error").unwrap().to_term(env);
            let message = err.description().encode(env);
            Ok(make_tuple(env, &[error, message]))
        }
    }
}
