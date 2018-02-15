#![allow(dead_code)]
#[macro_use] extern crate rustler;
#[macro_use] extern crate error_chain;
#[macro_use] extern crate lazy_static;
extern crate json;

use rustler::{Env, Term};
use rustler::schedule::SchedulerFlags::*;
use decoder::ParserResource;

mod decoder;
mod encoder;
mod errors;
mod parser;
mod sink;
mod util;
mod atoms;

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

fn load(env: Env, _info: Term) -> bool {
    resource_struct_init!(ParserResource, env);
    true
}
