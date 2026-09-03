use std::path::PathBuf;

use crate::Context;

pub(crate) async fn determine_file_type() {
    // Determine type of file to be processed
    // i.e. is it a chest/entity type texture - which warrants different process
}

pub(crate) async fn write_file() {
    // Write file to target destination with matching file structure
}

pub(crate) async fn process_file(file_path: &PathBuf, context: &Context) {
    // Determine file type (based off name and/or path)
    // Call relevant operation(s)
    // Write result
}

pub(crate) async fn process_directory(source_path: &PathBuf, context: &Context) {
    // Scan through files
    // If directory matches ignore - skip
    // If no match, recurse
    // Else process file (if valid type)
    // Else skip
}