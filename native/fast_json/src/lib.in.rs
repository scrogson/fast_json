#[macro_use]
extern crate rustler;
#[macro_use]
extern crate error_chain;
extern crate json;

use rustler::{NifEnv, NifTerm};
use rustler::atom::init_atom;

//mod errors;
mod encoder;
mod decoder;
mod parser;

rustler_export_nifs! {
    "Elixir.Json",
    [("native_parse", 2, decoder::decode),
     ("stringify", 2, encoder::encode)],
    Some(load)
}

fn load(_env: &NifEnv, _info: NifTerm) -> bool {
    init_atom("ok");
    init_atom("error");

    true
}
