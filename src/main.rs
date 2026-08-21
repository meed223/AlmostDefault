use std::{path::PathBuf, process::ExitCode};
use clap::{ArgMatches, parser::MatchesError};

pub mod cli;

#[tokio::main]
async fn main() -> ExitCode {
    let arg_matches = cli::build_cli().get_matches();

    let context = match create_context_from_args(&arg_matches) {
        Ok(context) => (),
        Err(error) => {
            println!("{0}", error.to_string());
            ExitCode::FAILURE;
        }
    };
    
    ExitCode::SUCCESS
}

fn create_context_from_args(matches: &ArgMatches) -> Result<Context, MatchesError> {
    let source_path = matches.try_get_one::<PathBuf>("input")?.unwrap();
    let output_path = matches.try_get_one::<PathBuf>("input")?.unwrap();
    let scale = matches.try_get_one::<i32>("scale")?.unwrap();
    let ignore_paths = matches.try_get_many::<PathBuf>("ignore")?;
    let a: Vec<PathBuf>;

    let context = Context {
        source_path: source_path.clone(), 
        output_path: output_path.clone(), 
        scale: scale.clone(), 
        ignore_paths: a.to_owned(),
    };

    Ok(context)
}


struct Context {
    source_path: PathBuf,
    output_path: PathBuf,
    scale: i32,
    ignore_paths: Vec<PathBuf>
}