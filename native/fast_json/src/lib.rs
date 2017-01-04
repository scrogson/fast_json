#![allow(dead_code)]
#[macro_use]
extern crate rustler;
#[macro_use]
extern crate error_chain;
extern crate json;

use rustler::{NifEnv, NifTerm};
use rustler::atom::init_atom;

mod decoder;
mod encoder;
mod errors;
mod parser;
mod sink;

use parser::ParserResource;

rustler_export_nifs! {
    "Elixir.Json",
    [("naive_parse", 2, decoder::naive_parse),
     ("stringify", 2, encoder::encode)],
    Some(load)
}

fn load(env: &NifEnv, _info: NifTerm) -> bool {
    init_atom("ok");
    init_atom("error");

    resource_struct_init!(ParserResource, env);

    true
}
