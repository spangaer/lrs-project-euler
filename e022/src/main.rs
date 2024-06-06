use e_tools::efile::{file_lines, make_available};
use std::path::Path;

fn main() {
    let alphabet = (b'A'..=b'Z').map(|c| c as char).collect::<Vec<_>>();

    let worth = |s: &str| {
        s.chars()
            .map(|c| alphabet.iter().position(|&a| a == c).unwrap() + 1)
            .sum::<usize>()
    };

    println!("{}", worth("COLIN"));

    let mut input = input();

    input.sort();

    // input.iter().for_each(|line| println!("{}", line));

    let res: usize = input
        .iter()
        .enumerate()
        .map(|(i, name)| (i + 1) * worth(name))
        .sum();

    println!("{}", res); //871198282
}

fn input() -> Vec<String> {
    let file_path = Path::new("names.txt");

    let _ = make_available(
        "https://projecteuler.net/resources/documents/0022_names.txt",
        file_path,
    )
    .unwrap();

    let input = file_lines(file_path).unwrap();

    input
        .iter()
        .filter(|line| line.len() > 0)
        .flat_map(|line| {
            line.split(',')
                .map(|quoted| String::from(quoted.trim_matches('"')))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}
