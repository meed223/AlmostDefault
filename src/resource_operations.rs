use std::{collections::HashMap, path::PathBuf};
use image::{ImageBuffer, Rgba, RgbaImage};
use tokio::fs::{self, File};

use crate::{image_manipulation::{circular_filter, get_mean_colour, median_upscale}, UpscalingParameters};

pub(crate) enum ResourceType {
    NonImage,
    Item,
    Block,
}

pub(crate) fn determine_resource_type(resources: &Vec<PathBuf>) -> Result<HashMap<PathBuf, ResourceType>, &'static str> {
    let mut type_map = HashMap::new();
    let mut filename;
    let mut path_as_string;

    for r in resources {
        filename = match r.extension() {
            Some(f) => f,
            None => return Err("Unable to resolve file extension.")
        };
        path_as_string = r.to_string_lossy();

        if filename != "png" || path_as_string.contains("colormap/") {
            type_map.insert(r.to_owned(), ResourceType::NonImage);
        } else {
            if path_as_string.contains("items/") {
                type_map.insert(r.to_owned(), ResourceType::Item);
            } else {
                type_map.insert(r.to_owned(), ResourceType::Block);
            }
        }
    }

    return Ok(type_map);
}

pub(crate) async fn copy_resource(read_root: &PathBuf, write_root: &PathBuf, resource: PathBuf) -> Result<(), &'static str> {
    match fs::copy(read_root.join(&resource), write_root.join(resource)).await {
        Ok(_u) => return Ok(()),
        Err(_e) => return Err("Error: Unable to copy resource.")
    };
}

pub(crate) async fn process_block_resource(resource: PathBuf, read_root: &PathBuf, write_root: &PathBuf, upscaling_parameters: &UpscalingParameters) -> Result<(), &'static str> {
    let source_img = match image::open(read_root.join(&resource)) {
        Ok(i) => RgbaImage::from(i),
        Err(_e) => return Err("Error: Unable to read image into buffer. (block)")
    };

    let mut intermediate_img: ImageBuffer<Rgba<u8>, Vec<u8>> = RgbaImage::new(source_img.width() + 2, source_img.height() + 2);
    let mut corner_colours: Vec<&Rgba<u8>> = Vec::new();
    let mut current_colour: &Rgba<u8>;

    for y in 1..source_img.height() + 1 {
        for x in 1..source_img.width() + 1 {
            current_colour = source_img.get_pixel(x-1, y-1);
            intermediate_img.put_pixel(x, y, current_colour.to_owned());

            if y == 1 || y == source_img.height() || x == 1 || x == source_img.height() {
                corner_colours.push(current_colour);
            }
        }
    }

    let mean_colour: Rgba<u8> = get_mean_colour(corner_colours).await;

    for y in 0..intermediate_img.height() {
        intermediate_img.put_pixel(0, y, mean_colour);
        intermediate_img.put_pixel(intermediate_img.width() - 1, y, mean_colour);
    }

    for x in 0..intermediate_img.width() {
        intermediate_img.put_pixel(x, 0, mean_colour);
        intermediate_img.put_pixel(x, intermediate_img.height() - 1, mean_colour);
    }

    let mut upscaled_img = match median_upscale(&intermediate_img, &upscaling_parameters).await {
        Ok(i) => i,
        Err(e) => return Err(e)
    };

    upscaled_img = match circular_filter(&intermediate_img, upscaled_img, upscaling_parameters).await {
        Ok(i) => i,
        Err(e) => return Err(e)
    };

    let mut trimmed_upscaled_img = RgbaImage::new(source_img.width() * upscaling_parameters.scale as u32, source_img.height() * upscaling_parameters.scale as u32);

    for y in 0..trimmed_upscaled_img.height() {
        for x in 0..trimmed_upscaled_img.width() {
            trimmed_upscaled_img.put_pixel(x, y, *upscaled_img.get_pixel(x + upscaling_parameters.scale as u32, y + upscaling_parameters.scale as u32))
        }
    }

    let file = write_root.join(&resource);
    match File::create(&file).await {
        Ok(f) => f,
        Err(_e) => {
            return Err("Error: Failed to create image file.")
        }
    };

    match trimmed_upscaled_img.save(&file) {
        Ok(x) => x,
        Err(_e) => return Err("Error: Failed to write image contents to file.")
    };

    return Ok(());
}

pub(crate) async fn process_item_resource(resource: PathBuf, read_root: &PathBuf, write_root: &PathBuf, upscaling_parameters: &UpscalingParameters) -> Result<(), &'static str> {
    let source_img = match image::open(read_root.join(&resource)) {
        Ok(i) => RgbaImage::from(i),
        Err(_e) => return Err("Error: Unable to read image into buffer. (item)")
    };

    let mut upscaled_img = match median_upscale(&source_img, &upscaling_parameters).await {
        Ok(i) => i,
        Err(e) => return Err(e)
    };

    upscaled_img = match circular_filter(&source_img, upscaled_img, upscaling_parameters).await {
        Ok(i) => i,
        Err(e) => return Err(e)
    };

    let file = write_root.join(&resource);
    match File::create(&file).await {
        Ok(f) => f,
        Err(_e) => {
            return Err("Error: Failed to create image file.")
        }
    };

    match upscaled_img.save(&file) {
        Ok(x) => x,
        Err(_e) => return Err("Error: Failed to write image contents to file.")
    };

    return Ok(());
}