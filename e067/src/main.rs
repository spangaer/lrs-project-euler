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

    println!("{}", res);
}

fn input() -> Vec<Vec<u64>> {
    todo!("get input from https://projecteuler.net/resources/documents/0067_triangle.txt")
    let input = "\
        75\n\
        95 64\n\
        17 47 82\n\
        18 35 87 10\n\
        20 04 82 47 65\n\
        19 01 23 75 03 34\n\
        88 02 77 73 07 63 67\n\
        99 65 04 28 06 16 70 92\n\
        41 41 26 56 83 40 80 70 33\n\
        41 48 72 33 47 32 37 16 94 29\n\
        53 71 44 65 25 43 91 52 97 51 14\n\
        70 11 33 28 77 73 17 78 39 68 17 57\n\
        91 71 52 38 17 14 91 43 58 50 27 29 48\n\
        63 66 04 68 89 53 67 30 73 16 69 87 40 31\n\
        04 62 98 27 23 09 70 98 73 93 38 53 60 04 23\
    ";

    input
        .split('\n')
        .map(|line| {
            line.split(' ')
                .map(|digits| u64::from_str_radix(digits, 10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}
