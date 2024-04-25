use std::{fs, path::{Path, PathBuf}};
use walkdir::WalkDir;

pub fn read_source_files(root_path: &PathBuf) -> Result<Vec<PathBuf>, &'static str> {
    let mut resource_rel_paths: Vec<PathBuf> = Vec::new();

    for entry in WalkDir::new(&root_path)
    .follow_links(false)
    .into_iter()
    .filter_map(Result::ok)
    .filter(|e| !e.file_type().is_dir()) {
        match entry.path().strip_prefix(&root_path) {
            Ok(path) => resource_rel_paths.push(path.to_owned()),
            Err(_e) => return Err("Error: Unable to form relative-path for resource.")
        }
    }

    return Ok(resource_rel_paths);
}

pub fn create_output_directory_structure(write_root: &PathBuf, resources: &Vec<PathBuf>) -> Result<(), &'static str> {
    let mut written_paths:Vec<&Path> = Vec::new();

    for r in resources {
        // ToDo: Try and join these two conditions into a single 'if'
        if let Some(parent_rel_path) = r.parent() {
            if written_paths.contains(&parent_rel_path) {
                let absolute_output_sub_dir = &write_root.join(parent_rel_path);

                match fs::create_dir(absolute_output_sub_dir) {
                    Ok(d) => d,
                    Err(_e) => return Err("Error: Unable to create directory in output structure.")
                }

                written_paths.push(parent_rel_path);
            }
        }
    }

    return Ok(());
}