use clap::{Command, arg, value_parser};
use clio::ClioPath;

pub(crate) fn build_cli() -> Command{
    Command::new("AlmostDefault")
        .about("Upscales Minecraft textures, making them less edgy.")
        .arg(arg!(
                "Input path": -i --input <PATH> "File path of texture or directory containing texture(s)."
            )
            .required(true)
            .value_parser(value_parser!(ClioPath).exists()),
        )
        .arg(arg!(
                "Output path": -o --output <PATH> "Output path to write texture(s). Default: \".\""
            )
            .required(false)
            .value_parser(value_parser!(ClioPath).exists().is_dir())
            .default_value("."),
        )
        .arg(arg!(
                "Upscaling multiplier": -x --scale <VALUE> "The upscaling multiplier. Accepted values: 4, 8 or 16. Default: 4"
            )
            .required(false)
            .value_parser(clap::builder::PossibleValuesParser::new(
                ["4", "8", "16"]
            ))
            .default_value("4"),
        )
        .arg(arg!(
                "Ignore paths": -n --ignore <VALUE> "Directories or file(s) to ignore"
            )
            .required(false)
            .value_parser(value_parser!(ClioPath).exists()),
        )
}