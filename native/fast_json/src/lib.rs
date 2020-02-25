#[macro_use]
extern crate error_chain;

use crate::decoder::ParserResource;
use rustler::{Env, Term};
use scoped_pool::Pool;

mod atoms;
mod decoder;
//mod encoder;
mod errors;
mod parser;
mod sink;
mod util;

lazy_static::lazy_static! {
    static ref POOL: Pool = Pool::new(num_cpus::get());
}

rustler::init! {
    "Elixir.Json.Native",
    [
        decoder::decode_naive,
        decoder::decode_init,
        decoder::decode_iter,
        decoder::decode_dirty,
        decoder::decode_threaded,
        //encoder::encode,
    ],
    load = load
}

fn load(env: Env, _info: Term) -> bool {
    rustler::resource!(ParserResource, env);
    true
}
