#![allow(dead_code)]
#[macro_use]
extern crate rustler;
#[macro_use]
extern crate error_chain;
extern crate json;

use rustler::{NifEnv, NifTerm};
use rustler::types::atom::init_atom;

mod decoder;
mod encoder;
mod errors;
mod parser;
mod sink;

use decoder::ParserResource;

rustler_export_nifs! {
    "Elixir.Json",
    [("naive_parse", 2, decoder::naive_parse),
     ("decode_init", 2, decoder::decode_init),
     ("decode_iter", 2, decoder::decode_iter),
     ("stringify", 2, encoder::encode)],
    Some(load)
}

fn load(env: &NifEnv, _info: NifTerm) -> bool {
    init_atom("ok");
    init_atom("error");
    init_atom("more");

    resource_struct_init!(ParserResource, env);

    true
}
