use thiserror::Error;

#[derive(Error, Debug)]
pub enum ShiftSizeError {
    #[error("value is bigger than 26")]
    TooBig,
}
