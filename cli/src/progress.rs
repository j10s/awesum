use anyhow::Error;
use indicatif::{MultiProgress, ProgressBar, ProgressBarIter, ProgressDrawTarget, ProgressStyle};
use lib::progress::Progress;
use std::{
    fs::File,
    io::{BufReader, Read},
    path::PathBuf,
};

pub struct CliProgress {
    pub multi: Option<MultiProgress>,
    pub total: Option<ProgressBar>,
}

impl CliProgress {
    pub fn new(show_progress: bool, len: Option<usize>) -> Self {
        let mut multi: Option<MultiProgress> = None;
        let mut total: Option<ProgressBar> = None;

        if show_progress {
            let m = MultiProgress::new();
            let t = get_total_progress_bar(len.unwrap_or(0));
            total = Some(m.add(t));
            multi = Some(m);
        }

        Self { total, multi }
    }
}

impl Progress for CliProgress {
    fn start(&self) {
        if let Some(total) = &self.total {
            total.tick();
        }
    }

    fn clear(&self) -> Result<(), Error> {
        if let Some(multi) = &self.multi {
            return Ok(multi.clear()?);
        }

        return Ok(());
    }

    fn get_reader(&self, path: &PathBuf) -> Result<Box<dyn Read>, Error> {
        let reader: Box<dyn Read> = if let Some(multi) = &self.multi {
            Box::new(get_progress_reader(&path, &multi)?)
        } else {
            Box::new(get_buf_reader(&path)?)
        };

        Ok(reader)
    }

    fn set_total_length(&self, len: usize) {
        if let Some(total) = &self.total {
            total.set_length(len as u64);

            if len <= 1 {
                total.set_draw_target(ProgressDrawTarget::hidden());
            }
        }
    }

    fn increment_total(&self) {
        if let Some(total) = &self.total {
            total.inc(1);
        }
    }
}

fn get_progress_reader(
    path: &PathBuf,
    mb: &MultiProgress,
) -> Result<ProgressBarIter<BufReader<File>>, Error> {
    let reader = get_buf_reader(path)?;
    let file_length = reader.get_ref().metadata()?.len();
    let pb = mb.add(get_progress_bar(path, file_length));
    return Ok(pb.wrap_read(reader));
}

//TODO replace with File::open_buffered when it stabilises
fn get_buf_reader(path: &PathBuf) -> Result<BufReader<File>, Error> {
    let file = File::open(path)?;
    return Ok(BufReader::new(file));
}

fn get_total_progress_bar(len: usize) -> ProgressBar {
    let style = ProgressStyle::with_template("[{wide_bar:.cyan/white}] {pos}/{len}")
        .unwrap()
        .progress_chars("##-");

    return ProgressBar::new(len as u64).with_style(style);
}

fn get_progress_bar(path: &PathBuf, file_length: u64) -> ProgressBar {
    let style =
        ProgressStyle::with_template("{spinner:.green} {wide_msg} {bytes}/{total_bytes}").unwrap();

    let message = path.display().to_string();
    return ProgressBar::new(file_length)
        .with_style(style)
        .with_message(message);
}
