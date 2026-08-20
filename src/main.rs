use std::{path::PathBuf, process::ExitCode};

pub mod cli;

#[tokio::main]
async fn main() -> ExitCode {
    let arg_matches = cli::build_cli().try_get_matches_from_mut(itr)

    ExitCode::SUCCESS
}

struct Context {
    source_path: PathBuf,
    output_path: PathBuf,
    scale: i32,
    ignore_paths: Vec<PathBuf>
}