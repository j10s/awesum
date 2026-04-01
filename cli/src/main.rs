mod args;
mod msg_handler;
mod progress;

use anyhow::Error;
use args::Args;
use clap::Parser;
use console::style;
use msg_handler::CliMessageHandler;
use progress::CliProgress;
use std::{cmp::min, process, sync::Arc};

fn main() -> Result<(), Error> {
    let args = Args::parse();

    rayon::ThreadPoolBuilder::new()
        .num_threads(min(args.jobs, num_cpus::get()))
        .build_global()?;

    if args.check {
        let status = *&args.status;
        let check_result = check(&args);
        if status {
            process::exit(get_return_code(&check_result));
        }
        return check_result;
    }

    return create(&args);
}

fn check(args: &Args) -> Result<(), Error> {
    warn_on_multiple_file_paths(args.files.len());

    // clap ensures at least 1 path
    let path = &args.files[0];
    let algorithm = args.algorithm; //TODO derive algorithm from path
    let show_progress = !args.quiet;
    let progress = get_progress(show_progress, None);
    let show_errors = !args.status;
    let show_warnings = args.warn;
    let msg_handler = get_msg_handler(progress.clone(), show_warnings, show_errors);

    let parsed_lines = lib::parse(path, algorithm, msg_handler.clone())?;
    return lib::check(&parsed_lines, algorithm, progress, msg_handler.clone());
}

fn create(args: &Args) -> Result<(), Error> {
    let paths = &args.files;
    let algorithm = args.algorithm;
    let show_progress = !args.quiet;
    let progress = get_progress(show_progress, Some(paths.len()));
    let msg_handler = get_msg_handler(progress.clone(), false, true);

    return lib::create(paths, algorithm, progress, msg_handler);
}

fn warn_on_multiple_file_paths(len: usize) {
    if len > 1 {
        let prefix = style("warning").bold().yellow();
        let message = "when check is set, only one file path is supported. using first file path";
        eprintln!("{prefix}: {message}");
    }
}

fn get_progress(show_progress: bool, len: Option<usize>) -> Arc<CliProgress> {
    return Arc::new(CliProgress::new(show_progress, len));
}

fn get_msg_handler(
    progress: Arc<CliProgress>,
    show_warnings: bool,
    show_errors: bool,
) -> Arc<CliMessageHandler> {
    return Arc::new(CliMessageHandler::new(progress, show_warnings, show_errors));
}

fn get_return_code(result: &Result<(), Error>) -> i32 {
    return match result {
        Ok(_) => 0,
        Err(_) => 1,
    };
}
