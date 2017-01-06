#![allow(dead_code)]
#[macro_use]
extern crate rustler;
#[macro_use]
extern crate error_chain;
extern crate json;

use rustler::{NifEnv, NifTerm};
use rustler::schedule::NifScheduleFlags::*;
use rustler::types::atom::init_atom;

mod decoder;
mod encoder;
mod errors;
mod parser;
mod sink;
mod util;

use decoder::ParserResource;

rustler_export_nifs! {
    "Elixir.Json",
    [("decode_naive", 2, decoder::decode_naive),
     ("decode_init", 2, decoder::decode_init),
     ("decode_iter", 2, decoder::decode_iter),
     ("decode_dirty", 2, decoder::decode_naive, DirtyCpu),
     ("decode_threaded", 2, decoder::decode_threaded),
     ("encode", 2, encoder::encode, DirtyCpu)],
    Some(load)
}

fn load(env: NifEnv, _info: NifTerm) -> bool {
    resource_struct_init!(ParserResource, env);

    init_atom("ok");
    init_atom("error");
    init_atom("more");

    true
}
