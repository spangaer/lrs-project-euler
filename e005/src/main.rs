use e_math::generate::Primes;

fn main() {
    let i = 20_usize;

    // println!("{}", Primes::<usize>::pow(2, 3));
    // println!("{}", Primes::<usize>::log(20, 2));
    // println!("{}", Primes::<usize>::log(10, 2));
    // println!("{}", Primes::<usize>::log(100, 10));
    // println!("{}", Primes::<usize>::log(1000, 10));

    // let mut primes = Primes::<usize>::new();

    // for x in 2..=20 {
    //     println!("{}: {:?}", x, primes.factorize_with_zeros(x));
    // }

    let numbers = (2..=i).collect::<Vec<_>>();

    let res = lcm(&numbers);

    println!("{}", res);
}

// lcm is max power of each prime factore
fn lcm(numbers: &[usize]) -> usize {
    let mut primes = Primes::<usize>::new();

    let factors = numbers
        .iter()
        .map(|&n| primes.factorize_with_zeros(n))
        .collect::<Vec<_>>();

    let max_len = factors.iter().map(|fr| fr.len()).max().unwrap();

    let mut lcm_factors = vec![];

    for i in 0..max_len {
        factors
            .iter()
            .map(|fr| fr.get(i).map(|f| f.clone()))
            .fold::<Option<(usize, usize)>, _>(None, |current, other| match (current, other) {
                // if only one of either defined that wins
                // if both defined the larges wins
                (x, None) => x,
                (None, y) => y,
                (Some(x), Some(y)) if x.1 >= y.1 => current, // avoid new object
                (_, y) => y,
            })
            .iter()
            .for_each(|&factor| lcm_factors.push(factor));
    }

    lcm_factors.iter().fold(1_usize, |current, (prime, pow)| {
        current * Primes::<usize>::pow(*prime, *pow)
    })
}
