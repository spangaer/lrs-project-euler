use std::fs::metadata;
use std::fs::File;
use std::io;
use std::io::copy;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::result::Result;

pub fn make_available<'a>(url: &str, file_path: &'a Path) -> Result<&'a Path, EFileError> {
    let file_size = match metadata(file_path) {
        Ok(meta) => meta.len(),
        Err(_) => 0_u64,
    };

    if file_size <= 0 {
        let _ = obtain(url, file_path)?;
    }

    Ok(file_path)
}

pub fn obtain(url: &str, file_path: &Path) -> Result<(), EFileError> {
    // Perform the HTTP GET request
    let response = reqwest::blocking::get(url)?;

    // Ensure the request was successful
    if !response.status().is_success() {
        panic!("Error: {}", response.status());
    }

    // Open a file to write the downloaded data
    let mut file = File::create(file_path)?;

    // Copy the response body to the file
    copy(&mut response.bytes().unwrap().as_ref(), &mut file)?;

    Ok(())
}

/// Allow merging 2 error types in to one using implicit conversion
#[derive(Debug)]
pub enum EFileError {
    RError(reqwest::Error),
    IOError(io::Error),
}

impl From<io::Error> for EFileError {
    fn from(err: io::Error) -> Self {
        EFileError::IOError(err)
    }
}

impl From<reqwest::Error> for EFileError {
    fn from(err: reqwest::Error) -> Self {
        EFileError::RError(err)
    }
}

pub fn file_lines(file_path: &Path) -> Result<Vec<String>, io::Error> {
    let f = File::open(file_path)?;

    let reader = BufReader::new(f);

    reader.lines().collect()
}
