use e_tools::log_timings;

fn main() {
    log_timings(|| {
        let target = 200;
        let coins: Vec<u64> = vec![1, 2, 5, 10, 20, 50, 100, 200];
        let limit: Vec<u64> = coins.iter().map(|c| target / c).collect();

        let mut count = 0_u64;

        let mut state: Vec<u64> = vec![0; coins.len()];

        loop {
            let mut i = 0;
            let mut update = true;

            while update {
                state[i] += 1;
                if state[i] <= limit[i] {
                    update = false; // we have a new valid state
                } else {
                    // need to increment higher coin
                    state[i] = 0; // first reset this coin
                    i += 1;
                }
            }

            let sum: u64 = state.iter().zip(coins.iter()).map(|(m, c)| m * c).sum();

            if sum == target {
                count += 1;

                if state.last() == limit.last() {
                    break; // end conditions is 1 time 2£
                }
            }

            if sum >= target {
                // adding more will not give new results, so short circuit state
                state[0] = limit[0];
            }
        }

        println!("{count}"); // 73682

        // debug: 11s 388ms
        // release: 72ms 497us
    })
}
