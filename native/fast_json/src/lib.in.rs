#[macro_use]
extern crate rustler;
extern crate json;

use rustler::{NifEnv, NifTerm};
use rustler::atom::init_atom;

mod encoder;
mod decoder;

rustler_export_nifs! {
    "Elixir.Json",
    [("native_parse", 2, decoder::decode),
     ("stringify", 2, encoder::encode)],
    Some(init)
}

fn init(_env: &NifEnv, _info: NifTerm) -> bool {
    init_atom("ok");
    init_atom("error");

    true
}
