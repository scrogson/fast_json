use rustler::{SchedulerFlags::*, Term};
use scoped_pool::Pool;

mod atoms;
mod decoder;
mod encoder;
mod error;
mod util;

lazy_static::lazy_static! {
    static ref POOL: Pool = Pool::new(num_cpus::get());
}

rustler::rustler_export_nifs! {
    "Elixir.Json",
    [
        ("decode_naive", 2, decoder::decode_naive),
        ("decode_dirty", 2, decoder::decode_naive, DirtyCpu),
        ("decode_threaded", 2, decoder::decode_threaded),
        ("decode_simd", 2, decoder::decode_simd, DirtyCpu),
        ("encode", 2, encoder::encode, DirtyCpu)
    ],
    None
}
