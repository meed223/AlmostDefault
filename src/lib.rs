use std::path::PathBuf;

pub mod cli;

pub struct Context {
    pub source_path: PathBuf,
    pub output_path: PathBuf,
    pub scale: i32,
    pub ignore_paths: Vec<PathBuf>
}

pub async fn process_file(file_path: &PathBuf, context: &Context) {
    // Determine file type (based off name and/or path)
    // Call relevant operation(s)
    // Write result
}

pub async fn process_directory(source_path: &PathBuf, context: &Context) {
    // Scan through files
    // If directory matches ignore - skip
    // If no match, recurse
    // Else process file (if valid type)
    // Else skip
}

pub async fn determine_file_type() {
    // Determine type of file to be processed
    // i.e. is it a chest/entity type texture - which warrants different process
}

pub async fn write_file() {
    // Write file to target destination with matching file structure
}

pub async fn pixel_doubling_upscale() {

}

pub async fn median_upscale() {

}

pub async fn median_upscale_with_corner_pass() {

}

pub async fn get_mean_colour() {

}

pub async fn circular_filter() {

}

pub async fn compare_ssse() {
    
}