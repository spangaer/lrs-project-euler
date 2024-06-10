use std::{
    iter::from_fn,
    sync::atomic::{fence, Ordering},
    time::Instant,
};

use humantime::format_duration;

fn main() {
    permutator(vec!['0', '1', '2']).for_each(|x| println!("{}", x));

    let symbols: Vec<char> = ('0'..='9').collect();

    let start = Instant::now();
    fence(Ordering::AcqRel);

    let p = permutator(symbols);
    let res = p.skip(999_999).next();
    println!("{:?}", res); // Some("2783915460")

    fence(Ordering::AcqRel); // prevent compiler tricks
    println!("{}", format_duration(start.elapsed()));

    // release: 214ms 29us
    // debug: 1s 782ms
}

fn permutator(symbols: Vec<char>) -> impl Iterator<Item = String> {
    let ubound = symbols.len();
    let ibound = ubound as _;
    let mut state: Vec<isize> = (0..ibound).collect();
    let last = {
        let mut c = state.clone();
        c.reverse();
        c
    };

    // to start the iteration, wee need to reset the second to last index
    // because that's the one we'll attempt to modify first
    state[ubound - 2] = -1;

    from_fn(move || {
        if state == last {
            None // emitted all permutations
        } else {
            // increment state
            {
                //mutating
                // start with second to last index
                // last has no freedom at all
                let mut m = ubound - 2;

                loop {
                    let opt = ((state[m] + 1)..ibound)
                        // cannot consider value of prior segments
                        .filter(|&i| state[0..m].iter().all(|&j| i != j))
                        .next();

                    match opt {
                        Some(i) => {
                            // we could select a new index for m

                            state[m] = i; // store it
                            m += 1; // switch to next segment

                            if m < ubound {
                                state[m] = -1; // reset segment to iterate from scratch
                            } else {
                                break; // we have settled on a new state
                            }
                        }
                        None => {
                            m -= 1;
                        }
                    }
                }
            }

            // emit new value
            let res: String = state.iter().map(|&i| symbols[i as usize]).collect();
            Some(res)
        }
    })
}
