use e_tools::efile::{file_lines, make_available};

use std::path::Path;
use std::thread;
use std::time::Duration;

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
    let file_path = Path::new("problem/0/0/6/e0067/triangle.txt");

    let _ = make_available(
        "https://projecteuler.net/resources/documents/0067_triangle.txt",
        file_path,
    )
    .unwrap();

    let input = file_lines(file_path).unwrap();

    input
        .iter()
        .filter(|line| line.len() > 0)
        .map(|line| {
            line.split(' ')
                .map(|digits| u64::from_str_radix(digits, 10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}
