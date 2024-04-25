use std::{collections::HashMap, fs, path::PathBuf};

use crate::UpscalingParameters;

pub(crate) enum ResourceType {
    NonImage,
    Item,
    Block,
}

pub(crate) fn determine_resource_type(resources: &Vec<PathBuf>) -> Result<HashMap<PathBuf, ResourceType>, &'static str> {
    let mut type_map = HashMap::new();

    for r in resources {
        if !r.ends_with(".png") {
            type_map.insert(r.to_owned(), ResourceType::NonImage);
        };

        if r.to_string_lossy().contains("items/") {
            type_map.insert(r.to_owned(), ResourceType::Item);
        } else {
            type_map.insert(r.to_owned(), ResourceType::Block);
        }
    }

    return Ok(type_map);
}

pub(crate) fn copy_resource(read_root: &PathBuf, write_root: &PathBuf, resource: PathBuf) -> Result<(), &'static str> {
    match fs::copy(read_root.join(&resource), write_root.join(resource)) {
        Ok(_u) => return Ok(()),
        Err(_e) => return Err("Error copying resource.")
    };
}

pub(crate) fn process_block_resource(resource: PathBuf, upscaling_parameters: &UpscalingParameters) -> Result<(), &'static str> {
    return Ok(());
}

pub(crate) fn process_item_resource(resource: PathBuf, upscaling_parameters: &UpscalingParameters) -> Result<(), &'static str> {
    return Ok(());
}