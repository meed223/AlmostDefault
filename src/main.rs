use std::{path::PathBuf, process::ExitCode};
use clap::{ArgMatches, parser::{MatchesError}};

use almostdefault::{cli::build_cli, *};

#[tokio::main]
async fn main() -> ExitCode {
    let arg_matches = build_cli().get_matches();

    let mut context: Context = Context { 
        source_path: PathBuf::new(), 
        output_path: PathBuf::new(), 
        scale: 0, 
        ignore_paths: Vec::new() 
    };
    
    match create_context_from_args(&arg_matches) {
        Ok(args_context) => context = args_context,
        Err(error) => {
            println!("{0}", error.to_string());
            return ExitCode::FAILURE;
        }
    };
    
    if context.source_path.is_file() {
        process_file(&context.source_path, &context).await;
    } else {
        process_directory(&context.source_path, &context).await;
    }

    return ExitCode::SUCCESS
}

fn create_context_from_args(matches: &ArgMatches) -> Result<Context, MatchesError> {
    let source_path = matches.try_get_one::<PathBuf>("input")?.unwrap();
    let output_path = matches.try_get_one::<PathBuf>("input")?.unwrap();
    let scale = matches.try_get_one::<i32>("scale")?.unwrap();
    let ignore_paths_option = matches.try_get_many::<PathBuf>("ignore")?;
    
    let mut ignore_paths_vec: Vec<PathBuf> = Vec::new();
    
    match ignore_paths_option {
        Some(paths) => {
            let paths_iter = paths.into_iter();
            ignore_paths_vec = paths_iter.map(|p| p.to_owned()).collect();
        }
        None => (),
    }

    let context = Context {
        source_path: source_path.to_owned(), 
        output_path: output_path.to_owned(), 
        scale: scale.to_owned(), 
        ignore_paths: ignore_paths_vec,
    };

    Ok(context)
}


