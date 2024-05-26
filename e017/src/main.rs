use english_numbers::convert;
use english_numbers::Formatting;

fn main() {
    let fmt = Formatting {
        title_case: false,
        spaces: false,
        conjunctions: true,
        commas: false,
        dashes: false,
    };

    (1..150_i64).for_each(|i| {
        println!("{}", convert(i, fmt));
    });

    let res = (1..=1000_i64)
        .map(|i| convert(i, fmt).chars().count())
        .sum::<usize>();

    println!("{}", res); //21124
}
