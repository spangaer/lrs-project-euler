use e_math::primesg::Primes;

fn main() {
    let top = 10000_u64;
    let mut primes = Primes::<u64>::new();

    for i in [1, 2, 220, 284] {
        let divisors = primes.divisors(i);
        println!("{:?}", divisors);

        let sum = divisors[0..divisors.len() - 1].iter().sum::<u64>();
        println!("{:?}", sum);
    }

    let mut d = |n| {
        let divisors = primes.divisors(n);
        divisors[0..divisors.len() - 1].iter().sum::<u64>()
    };

    for i in [2, 220, 284] {
        println!("{:?}", d(i));
    }

    let res = (2..top)
        .filter(|&x| {
            let d_x = d(x);

            let check = d_x < top && d_x != x && d(d_x) == x;
            if check {
                println!("{}  {}", x, d_x);
            }
            check
        })
        .sum::<u64>();

    println!("{}", res); //31626
}
