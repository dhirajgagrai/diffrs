use std::fs::{self, File};
use std::io::{Error, Read};
use std::path::PathBuf;

pub fn get_file_path(file: &String) -> Result<PathBuf, Error> {
    let dir = PathBuf::from(file);
    fs::canonicalize(&dir)
}

pub fn get_file_content(file: &String) -> Result<String, Error> {
    let path = get_file_path(file)?;

    let mut file = File::open(path).expect("Unable to open the file");
    let mut content = String::new();
    file.read_to_string(&mut content)
        .expect("Unable to read the file");

    Ok(content)
}
