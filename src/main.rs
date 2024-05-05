use std::{path::PathBuf, process::ExitCode};
use clap::Parser;
use fs_operations::{create_output_directory_structure, read_source_files};
use resource_operations::{copy_resource, determine_resource_type, process_block_resource, process_item_resource, ResourceType};

pub mod fs_operations;
pub mod resource_operations;
pub mod image_manipulation;

#[derive(Parser)]
#[command(about = "Upscales resource-pack textures, making them less edgy.", long_about = None)]
struct Args {
    #[arg(short = 'i', long = "input")]
    input: String,
    
    #[arg(short = 'o', long = "output",)]
    output: String,

    #[arg(short = 'x', long = "scale", default_value_t = 4)]
    scale: i32
}

fn main() -> ExitCode {
    let args: Args = Args::parse();

    let read_root_path = PathBuf::from(args.input);
    let resources = match read_source_files(&read_root_path) {
        Ok(r) => r,
        Err(msg) => {
            println!("{0}", msg);
            return ExitCode::FAILURE
        }
    };

    let write_root_path = PathBuf::from(args.output);
    match create_output_directory_structure(&write_root_path, &resources) {
        Ok(()) => (),
        Err(msg) => {
            println!("{0}", msg);
            return ExitCode::FAILURE
        }
    }

    let mapped_resources = match determine_resource_type(&resources) {
        Ok(r) => r,
        Err(msg) => {
            println!("{0}", msg);
            return ExitCode::FAILURE
        }
    };

    let upscaling_parameters = match get_upscaling_parameters(args.scale) {
        Ok(u) => u,
        Err(msg) => {
            println!("{0}", msg);
            return ExitCode::FAILURE
        }
    };

    let total_resources = mapped_resources.capacity() as i32;
    let mut resource_processed_count = 1;

    for r in mapped_resources {
        let result = match r.1 {
            ResourceType::NonImage => copy_resource(&read_root_path, &write_root_path, r.0),
            ResourceType::Block => process_block_resource(r.0, &read_root_path, &write_root_path, &upscaling_parameters),
            ResourceType::Item => process_item_resource(r.0, &read_root_path, &write_root_path, &upscaling_parameters)
        };

        print!("\rProcessed {0} of {1} resources", resource_processed_count, total_resources);
        resource_processed_count += 1;

        match result {
            Ok(()) => (),
            Err(msg) => {
                println!("{0}", msg);
                return ExitCode::FAILURE
            }
        }
    }
    println!("All resources processed!");
    ExitCode::SUCCESS
}

fn get_upscaling_parameters(scale: i32) -> Result<UpscalingParameters, &'static str> {
    match scale {
        4 => return Ok(UpscalingParameters { scale: 4, median: 3}),
        8 => return Ok(UpscalingParameters { scale: 8, median: 5}),
        16 => return Ok(UpscalingParameters { scale: 16, median: 9}),
        _ => return Err("Error: Unsupported scale. Please choose 4, 8 or 16.")
    }
}

struct UpscalingParameters {
    scale: i32,
    median: i32
}