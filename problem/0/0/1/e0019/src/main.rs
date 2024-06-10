use chrono::Weekday::Sun;
use chrono::{Datelike, Months, TimeZone, Utc};

fn main() {
    let start = Utc.with_ymd_and_hms(1901, 1, 1, 11, 55, 00).unwrap();
    let delta = Months::new(1);
    let end = Utc.with_ymd_and_hms(2000, 12, 31, 11, 55, 00).unwrap();

    println!("{}", start);
    println!("{}", end);

    let mut count = 0_usize;
    let mut state = start.clone();

    while state < end {
        if state.weekday() == Sun {
            count += 1
        }

        state = state.checked_add_months(delta).unwrap();
    }

    println!("{}", count); // 171
}
