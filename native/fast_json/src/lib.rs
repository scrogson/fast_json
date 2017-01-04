#![allow(dead_code)]
#[macro_use]
extern crate rustler;
#[macro_use]
extern crate error_chain;
extern crate json;

extern crate erlang_nif_sys;

use rustler::{NifEnv, NifTerm};
use rustler::atom::init_atom;

mod decoder;
mod encoder;
mod errors;
mod parser;
mod sink;
mod threaded_awesomeness;

use decoder::ParserResource;

rustler_export_nifs! {
    "Elixir.Json",
    [("naive_parse", 2, decoder::naive_parse),
     ("decode_init", 2, decoder::decode_init),
     ("decode_iter", 2, decoder::decode_iter),
     ("decode_threaded", 2, decoder::decode_threaded),
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
