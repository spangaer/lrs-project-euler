use e_math::generate::Primes;

fn main() {
    let i = 20;
    let primes = {
        let mut primes = Primes::<usize>::new();
        primes
            .iterator()
            .take_while(|&n| n <= i)
            .collect::<Vec<_>>()
    };

    // only need to divide if there's no higher multiple
    let dividers = (1_usize..=i)
        .filter(|x| ((x + 1)..=i).all(|y| y % x != 0))
        .collect::<Vec<_>>();
    println!("{:?}", dividers);

    let mut test = primes.iter().fold(1, |a, b| a * b);

    println!("{}", test);

    while !(dividers.iter().all(|&d| test % d == 0)) {
        test += 1
    }

    println!("{}", test)
}
