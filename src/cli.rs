use std::path::PathBuf;
use clap::{Command, arg, value_parser};

pub(crate) fn build_cli() -> Command{
    Command::new("AlmostDefault")
        .about("Upscales Minecraft textures, making them less edgy.")
        .arg(arg!(
                "Input path": --input <PATH> "File path of texture or directory containing texture(s)."
            )
            .required(true)
            .value_parser(value_parser!(PathBuf)),
        )
        .arg(arg!(
                "Output path": --output <PATH> "Output path to write texture(s). Default: \".\""
            )
            .required(false)
            .value_parser(value_parser!(PathBuf))
            .default_value("."),
        )
        .arg(arg!(
                "Upscaling multiplier": --scale <VALUE> "The upscaling multiplier. Accepted values: 4, 8 or 16. Default: 4"
            )
            .required(false)
            .value_parser(clap::builder::PossibleValuesParser::new(
                ["4", "8", "16"]
            ))
            .default_value("4"),
        )
        .arg(arg!(
                "Ignore paths": --ignore <VALUE> "Directories or file(s) to ignore"
            )
            .required(false)
            .value_parser(value_parser!(PathBuf)),
        )
}