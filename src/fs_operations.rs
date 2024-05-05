use std::{fs, path::PathBuf};
use walkdir::WalkDir;

pub fn read_source_files(root_path: &PathBuf) -> Result<Vec<PathBuf>, &'static str> {
    let mut resource_rel_paths: Vec<PathBuf> = Vec::new();
    let mut path_as_string;

    for entry in WalkDir::new(&root_path)
    .follow_links(false)
    .into_iter()
    .filter_map(Result::ok)
    .filter(|e| !e.file_type().is_dir()) {
        match entry.path().strip_prefix(&root_path) {
            Ok(path) => {
                path_as_string = path.to_string_lossy();
                // These paths contain files that don't need to be copied or upscaled
                if path_as_string.contains("realms") 
                || path_as_string.contains("title") 
                || path_as_string.contains("presets") {
                    continue;
                }
                resource_rel_paths.push(path.to_owned())
            },
            Err(_e) => return Err("Error: Unable to form relative-path for resource.")
        }
    }

    return Ok(resource_rel_paths);
}

pub fn create_output_directory_structure(write_root: &PathBuf, resources: &Vec<PathBuf>) -> Result<(), &'static str> {
    for r in resources {
        // ToDo: Try and join these two conditions into a single 'if'
        if let Some(parent_rel_path) = r.parent() {
            let absolute_output_sub_dir = &write_root.join(parent_rel_path);
            if !absolute_output_sub_dir.exists() {
                match fs::create_dir(absolute_output_sub_dir) {
                    Ok(d) => d,
                    Err(_e) => return Err("Error: Unable to create directory in output structure.")
                }
            }
        }
    }

    return Ok(());
}