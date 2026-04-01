use clap::{
    builder::{PossibleValuesParser, TypedValueParser},
    Parser,
};
use lib::algorithm::Algorithm;
use std::path::PathBuf;
use strum::VariantNames;

#[derive(Parser, Debug)]
#[command(version)]
/// Print or check checksums.
pub struct Args {
    /// One or more file paths. When check is set, only one file path is supported
    #[arg(value_name="FILE", num_args = 1..)]
    pub files: Vec<PathBuf>,

    /// Hashing algorithm to use
    #[arg(short, long, value_parser = algorithm_value_parser())]
    pub algorithm: Algorithm,

    /// Maximum number of files to hash at once
    #[arg(short, long, default_value_t = num_cpus::get(), hide_default_value = true)]
    pub jobs: usize,

    /// Check sums against given list
    #[arg(short, long, default_value_t = false)]
    pub check: bool,

    /// Don't fail or report status for missing files
    #[arg(short, long, default_value_t = false, requires = "check")]
    pub ignore_missing: bool,

    /// Don't show progress
    #[arg(short, long, default_value_t = false)]
    pub quiet: bool,

    /// Don't output anything, status code shows success
    #[arg(short, long, default_value_t = false, requires = "check")]
    pub status: bool,

    /// Warn about improperly formatted checksum lines
    #[arg(short, long, default_value_t = false, requires = "check")]
    pub warn: bool,
}

fn algorithm_value_parser() -> impl TypedValueParser {
    return PossibleValuesParser::new(Algorithm::VARIANTS).map(|s| s.parse::<Algorithm>().unwrap());
}
