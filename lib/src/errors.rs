use thiserror::Error;

#[derive(Error, Debug)]
pub enum CheckError {
    #[error("Checksum mismatch")]
    Mismatch,
    #[error("There were {0} bad files")]
    Aggregate(usize),
}

#[derive(Error, Debug)]
pub enum CreateError {
    #[error("There were {0} bad files")]
    Aggregate(usize),
}
