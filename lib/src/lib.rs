pub mod algorithm;
mod check;
mod create;
mod errors;
mod hashing;
pub mod msg_handler;
mod parse;
pub mod progress;

use algorithm::Algorithm;
use anyhow::Error;
use check::*;
use create::*;
use msg_handler::MsgHandler;
use parse::*;
use progress::Progress;
use std::{path::PathBuf, sync::Arc};

pub struct ParsedChecksumLine {
    pub path: PathBuf,
    pub hash: String,
}

pub fn parse<M: MsgHandler + 'static>(
    path: &PathBuf,
    algorithm: Algorithm,
    msg_handler: Arc<M>,
) -> Result<Vec<ParsedChecksumLine>, Error> {
    let state = ParseState {
        path,
        algorithm,
        msg_handler,
    };

    return parse_lines(&state);
}

pub fn check<P: Progress + 'static, M: MsgHandler + 'static>(
    lines: &[ParsedChecksumLine],
    algorithm: Algorithm,
    progress: Arc<P>,
    msg_handler: Arc<M>,
) -> Result<(), Error> {
    progress.set_total_length(lines.len());

    let state = CheckState {
        lines,
        algorithm,
        progress,
        msg_handler,
    };

    return check_lines(&state);
}

pub fn create<P: Progress + 'static, M: MsgHandler + 'static>(
    paths: &[PathBuf],
    algorithm: Algorithm,
    progress: Arc<P>,
    msg_handler: Arc<M>,
) -> Result<(), Error> {
    let state = CreateState {
        paths: &paths,
        algorithm,
        progress,
        msg_handler,
    };

    return hash_files(&state);
}
