use std::fs::File;
use std::fs::{metadata, read_to_string};
use std::io::copy;
use std::path::Path;
use std::result::Result;
use std::time::Duration;
use std::{io, thread};

fn main() {
    let input = input();

    input.iter().for_each(|line| println!("{:?}", line));

    let res = *input
        .iter()
        .fold(vec![0_u64; 0], |prev, line| {
            let out = if prev.is_empty() {
                line.clone()
            } else {
                let mut collect = vec![0_u64; 0];

                collect.push(line[0] + prev[0]);

                (1..line.len() - 1)
                    .for_each(|i| collect.push(u64::max(line[i] + prev[i - 1], line[i] + prev[i])));

                collect.push(line[line.len() - 1] + prev[line.len() - 2]);

                collect
            };
            println!("{:?}", out);

            out
        })
        .iter()
        .max()
        .unwrap();

    thread::sleep(Duration::from_secs(1));

    println!("{}", res); //7273
}

fn input() -> Vec<Vec<u64>> {
    let file_path = Path::new("triangle.txt");

    let file_size = match metadata(file_path) {
        Ok(meta) => meta.len(),
        Err(_) => 0_u64,
    };

    if file_size <= 0 {
        let _ = download_file(
            "https://projecteuler.net/resources/documents/0067_triangle.txt",
            file_path,
        )
        .expect("failed to download");
    }

    let input = read_to_string(file_path).unwrap();

    input
        .split('\n')
        .filter(|line| line.len() > 0)
        .map(|line| {
            line.split(' ')
                .map(|digits| u64::from_str_radix(digits, 10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

fn download_file(url: &str, file_path: &Path) -> Result<(), LocalError> {
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
enum LocalError {
    RError(reqwest::Error),
    IOError(io::Error),
}

impl From<io::Error> for LocalError {
    fn from(err: io::Error) -> Self {
        LocalError::IOError(err)
    }
}

impl From<reqwest::Error> for LocalError {
    fn from(err: reqwest::Error) -> Self {
        LocalError::RError(err)
    }
}
