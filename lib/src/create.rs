use crate::{algorithm::*, errors::CreateError, hashing::*, msg_handler::MsgHandler, progress::*};
use anyhow::Error;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::{
    path::PathBuf,
    sync::{
        atomic::{AtomicUsize, Ordering::Relaxed},
        Arc,
    },
};

pub struct CreateState<'a, P: Progress, M: MsgHandler> {
    pub paths: &'a [PathBuf],
    pub algorithm: Algorithm,
    pub progress: Arc<P>,
    pub msg_handler: Arc<M>,
}

pub fn hash_files<P: Progress, M: MsgHandler>(state: &CreateState<P, M>) -> Result<(), Error> {
    let CreateState {
        paths,
        algorithm,
        progress,
        msg_handler,
    } = state;
    let errors_atomic = AtomicUsize::new(0);
    progress.start();

    paths.par_iter().for_each(|path| {
        let reader = progress.get_reader(path);
        let result = reader.and_then(|mut r| hash(&mut r, *algorithm));

        match result {
            Ok(hash) => {
                let path_display = path.display();
                let message = match algorithm {
                    Algorithm::CRC32 => format!("{} {}", hash, path_display),
                    _ => format!("{} {}", path_display, hash),
                };
                msg_handler.show_message(&message);
            }
            Err(err) => {
                errors_atomic.fetch_add(1, Relaxed);
                msg_handler.show_hash_error(path, err);
            }
        };

        progress.increment_total();
    });

    progress.clear()?;

    let error_count = errors_atomic.load(Relaxed);
    return match error_count <= 0 {
        true => Ok(()),
        false => Err(CreateError::Aggregate(error_count).into()),
    };
}
