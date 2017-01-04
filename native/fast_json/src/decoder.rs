use ::parser;
use rustler::{NifDecoder, NifEncoder, NifEnv, NifTerm, NifResult};
use rustler::tuple::make_tuple;
use rustler::atom::get_atom;

pub fn naive_parse<'a>(env: &'a NifEnv, args: &Vec<NifTerm>) -> NifResult<NifTerm<'a>> {
    let data = <String as NifDecoder>::decode(args[0])?;

    match parser::naive_parse(env, data) {
        Ok(term) => {
            let ok = get_atom("ok").unwrap().to_term(env);
            Ok(make_tuple(env, &[ok, term]))
        }
        Err(err) =>  {
            let error = get_atom("error").unwrap().to_term(env);
            let message = format!("{}", err).encode(env);
            Ok(make_tuple(env, &[error, message]))
        }
    }
}

//pub fn chuck_parse<'a>(env: &'a NifEnv, args: &Vec<NifTerm>) -> NifResult<NifTerm<'a>> {
    //let data = <String as NifDecoder>::decode(args[0])?;

    //match parser::chuck_parse(env, data) {
        //Ok(term) => {
            //let ok = get_atom("ok").unwrap().to_term(env);
            //Ok(make_tuple(env, &[ok, term]))
        //}
        //Err(err) =>  {
            //let error = get_atom("error").unwrap().to_term(env);
            //let message = format!("{}", err).encode(env);
            //Ok(make_tuple(env, &[error, message]))
        //}
    //}
//}
