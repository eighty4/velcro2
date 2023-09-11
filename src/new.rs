use std::io::Write;

use crate::util::error_exit;

pub fn create_new_resource() {
    let filename = "mappings/mapping.json";
    let mut file = match std::fs::File::open(filename) {
        Ok(f) => f,
        Err(err) => error_exit(format!("error creating file {filename}: {err}")),
    };
    match file.write_all(include_bytes!("mapping.json")) {
        Ok(_) => println!("mappings/mapping.json created"),
        Err(err) => error_exit(format!("error writing to file {filename}: {err}")),
    }
}
