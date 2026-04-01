use anyhow::Error;
use std::{io::Read, path::PathBuf};

pub trait Progress: Send + Sync {
    fn start(&self);
    fn clear(&self) -> Result<(), Error>;
    fn set_total_length(&self, len: usize);
    fn increment_total(&self);
    fn get_reader(&self, path: &PathBuf) -> Result<Box<dyn Read>, Error>;
}
