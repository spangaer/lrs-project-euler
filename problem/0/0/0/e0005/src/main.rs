use e_math::primesg::Primes;

fn main() {
    let i = 20_u64;

    // println!("{}", Primes::<usize>::pow(2, 3));
    // println!("{}", Primes::<usize>::log(20, 2));
    // println!("{}", Primes::<usize>::log(10, 2));
    // println!("{}", Primes::<usize>::log(100, 10));
    // println!("{}", Primes::<usize>::log(1000, 10));

    let mut primes = Primes::<u64>::new();

    for x in 2..=20 {
        println!("{}: {:?}", x, primes.factorize_with_zeros(x));
    }

    let numbers = (2..=i).collect::<Vec<_>>();

    let res = primes.lcm(&numbers);

    println!("{}", res); //232792560
}
