use thiserror::Error;

#[derive(Error, Debug)]
pub enum ShiftSizeError {
    #[error("key is bigger than 26")]
    TooBig,
}
