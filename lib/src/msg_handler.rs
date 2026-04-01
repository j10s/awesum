use anyhow::Error;
use std::path::PathBuf;

pub trait MsgHandler: Send + Sync {
    fn show_message(&self, message: &str);
    fn show_warning(&self, message: &str);
    fn show_improper_format_warning(&self, line_number: usize);
    fn show_error(&self, message: &str);
    fn show_hash_error(&self, path: &PathBuf, error: Error);
}
