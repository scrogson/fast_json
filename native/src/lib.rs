#[macro_use]
extern crate error_chain;
extern crate json;
#[macro_use]
extern crate lazy_static;
extern crate num_cpus;
#[macro_use]
extern crate rustler;
extern crate scoped_pool;

use rustler::{Env, Term};
use rustler::schedule::SchedulerFlags::*;
use scoped_pool::Pool;

use decoder::ParserResource;

mod atoms;
mod decoder;
mod encoder;
mod errors;
mod parser;
mod sink;
mod util;

lazy_static! {
    static ref POOL: Pool = Pool::new(num_cpus::get());
}

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
