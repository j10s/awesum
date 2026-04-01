use crate::progress::CliProgress;
use anyhow::Error;
use console::style;
use lib::msg_handler::MsgHandler;
use std::{path::PathBuf, sync::Arc};

pub struct CliMessageHandler {
    progress: Arc<CliProgress>,
    show_warnings: bool,
    show_errors: bool,
}

impl CliMessageHandler {
    pub fn new(progress: Arc<CliProgress>, show_warnings: bool, show_errors: bool) -> Self {
        CliMessageHandler {
            progress,
            show_warnings,
            show_errors,
        }
    }
}

impl MsgHandler for CliMessageHandler {
    fn show_message(&self, message: &str) {
        if let Some(multi) = &self.progress.multi {
            multi.suspend(|| println!("{message}"));
        } else {
            println!("{message}");
        }
    }

    fn show_warning(&self, message: &str) {
        if !self.show_warnings {
            return;
        }

        let prefix = style("warning").bold().yellow();
        let warning = format!("{prefix}: {message}");

        if let Some(multi) = &self.progress.multi {
            multi.suspend(|| eprintln!("{warning}"));
        } else {
            eprintln!("{warning}");
        }
    }

    fn show_improper_format_warning(&self, line_number: usize) {
        let message = format!("{}: improperly formatted checksum line", line_number);
        self.show_warning(&message);
    }

    fn show_error(&self, message: &str) {
        if !self.show_errors {
            return;
        }

        let prefix = style("error").bold().red();
        let error = format!("{prefix}: {message}");

        if let Some(multi) = &self.progress.multi {
            multi.suspend(|| eprintln!("{error}"));
        } else {
            eprintln!("{error}");
        }
    }

    fn show_hash_error(&self, path: &PathBuf, error: Error) {
        let path_display = path.display();
        let error_message = error.to_string();
        let message = format!("{path_display} ({error_message})");
        self.show_error(&message);
    }
}
