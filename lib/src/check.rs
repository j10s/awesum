use crate::{algorithm::*, errors::*, hashing::*, msg_handler::*, progress::*, ParsedChecksumLine};
use anyhow::Error;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::{
    io::Read,
    sync::{
        atomic::{AtomicUsize, Ordering::Relaxed},
        Arc,
    },
};

pub struct CheckState<'a, P: Progress, M: MsgHandler> {
    pub lines: &'a [ParsedChecksumLine],
    pub algorithm: Algorithm,
    pub progress: Arc<P>,
    pub msg_handler: Arc<M>,
}

pub fn check_lines<P: Progress, M: MsgHandler>(state: &CheckState<P, M>) -> Result<(), Error> {
    let CheckState {
        lines,
        algorithm,
        progress,
        msg_handler,
    } = state;
    let mismatches_atomic = AtomicUsize::new(0);

    progress.start();

    lines.par_iter().for_each(|line| {
        let reader = progress.get_reader(&line.path);
        // TODO if reader is error then continue if --ignore-missing is set
        let result = reader.and_then(|mut r| check_line(&mut r, line, *algorithm));

        if let Err(err) = result {
            mismatches_atomic.fetch_add(1, Relaxed);
            msg_handler.show_hash_error(&line.path, err);
        }

        progress.increment_total();
    });

    progress.clear()?;

    let mismatch_count = mismatches_atomic.load(Relaxed);
    return match mismatch_count <= 0 {
        true => Ok(()),
        false => Err(CheckError::Aggregate(mismatch_count).into()),
    };
}

fn check_line<R: Read>(
    reader: &mut R,
    line: &ParsedChecksumLine,
    algorithm: Algorithm,
) -> Result<(), Error> {
    let expected = &line.hash;
    let hash = hash(reader, algorithm)?;
    let result = check_hash(&hash, &expected)?;

    return Ok(result);
}

fn check_hash(hash: &str, expected: &str) -> Result<(), CheckError> {
    if hash == expected {
        return Ok(());
    }

    return Err(CheckError::Mismatch);
}
