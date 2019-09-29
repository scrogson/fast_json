use failure::Fail;

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "{} at position {}", _0, _1)]
    InvalidJson { message: String, offset: usize },
}
