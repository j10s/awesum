use crate::{algorithm::*, msg_handler::*, ParsedChecksumLine};
use anyhow::Error;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
    sync::Arc,
};

pub struct ParseState<'a, M: MsgHandler> {
    pub path: &'a PathBuf,
    pub algorithm: Algorithm,
    pub msg_handler: Arc<M>,
}

pub fn parse_lines<M: MsgHandler>(state: &ParseState<M>) -> Result<Vec<ParsedChecksumLine>, Error> {
    let ParseState {
        path,
        algorithm,
        msg_handler,
    } = state;

    let dir = get_directory(path);
    let reader = get_buf_reader(path)?;
    let lines = reader.lines();

    let mut results: Vec<ParsedChecksumLine> = Vec::new();
    for (i, line) in lines.enumerate() {
        let line_number = i + 1;
        let utf8_line = match line {
            Ok(x) => x,
            Err(_) => {
                msg_handler.show_improper_format_warning(line_number);
                continue;
            }
        };

        if utf8_line.starts_with(';') {
            continue;
        }

        let result = parse_line(&utf8_line, *algorithm, &dir);
        match result {
            Some(parsed_checksum_line) => results.push(parsed_checksum_line),
            None => msg_handler.show_improper_format_warning(line_number),
        };
    }

    return Ok(results);
}

fn get_directory(path: &PathBuf) -> PathBuf {
    let directory = match path.parent() {
        Some(dir) => dir,
        None => path,
    };

    return directory.to_path_buf();
}

//TODO replace with File::open_buffered when it stabilises
fn get_buf_reader(path: &PathBuf) -> Result<BufReader<File>, Error> {
    let file = File::open(path)?;
    return Ok(BufReader::new(file));
}

fn parse_line(line: &str, alg: Algorithm, dir: &PathBuf) -> Option<ParsedChecksumLine> {
    if let Some((path, hash)) = split_line(line, alg) {
        return Some(ParsedChecksumLine {
            path: dir.join(path.trim_start_matches('*').trim()),
            hash: String::from(hash.trim()),
        });
    }

    return None;
}

fn split_line<'a>(line: &'a str, alg: Algorithm) -> Option<(&'a str, &'a str)> {
    if matches!(alg, Algorithm::CRC32) {
        return line.rsplit_once(' ');
    } else {
        if let Some((hash, path)) = line.split_once(' ') {
            return Some((path, hash));
        }

        return None;
    }
}
