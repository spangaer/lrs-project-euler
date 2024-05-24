use std::collections::HashMap;

/*
Studied for i = 1, 2, 3 and 4.

2 1
1 1

6 3 1
3 2 1
1 1 1

20 10  4  1
10  6  3  1
 4  3  2  1
 1  1  1  1

70 35 15  5  1
35 20 10  4  1
15 10  6  3  1
 5  4  3  2  1
 1  1  1  1  1

I guess som sort of pyramid numbers.
If r and c are row and column starting at the bottom with (1,1).
Then f(1,_) = 1 and f(_, 1) = 1 and f(r,c) = f(r - 1, c) + f(r, c-1).
Finally n(i) = f(i+1,i+1)
*/
fn main() {
    // sadly, implementing it stateless, gives combinatorial explosion
    // so we need to hold on to known values
    let mut cache: HashMap<(u128, u128), u128> = HashMap::new();

    fn f(cache: &mut HashMap<(u128, u128), u128>, r: u128, c: u128) -> u128 {
        match (r, c) {
            (1, _) => 1,
            (_, 1) => 1,
            (r, c) if c > r => f(cache, c, r), // switch c and r
            key @ (r, c) => match cache.get(&key) {
                Some(res) => *res,
                None => {
                    let res = f(cache, r - 1, c) + f(cache, r, c - 1);
                    cache.insert(key, res);
                    res
                }
            },
        }
    }

    let mut n = |i| f(&mut cache, i + 1, i + 1);

    for i in [1, 2, 3, 4, 20_u128] {
        println!("{}:  {}", i, n(i));
    }

    /*
    1:  2
    2:  6
    3:  20
    4:  70
    20:  137846528820
    */
}
