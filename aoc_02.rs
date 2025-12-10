use std::error::Error;
use std::ops::Index;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref INPUT: Vec<&'static str> = include_str!("../input/02.txt").split(",").collect();

    pub static ref TEST_INPUT: Vec<&'static str> = vec![
        "11-22",
        "95-115",
        "998-1012",
        "1188511880-1188511890",
        "222220-222224",
        "1698522-1698528",
        "446443-446449",
        "38593856-38593862",
        "565653-565659",
        "824824821-824824827",
        "2121212118-2121212124"
    ];
}

pub fn solve(data: &Vec<&str>) -> u64 {
    let mut invalid_id_sum = 0;

    for range_str in data.iter() {
        // 1. Parse the bounds
        let mut parts = range_str.split('-');
        let first_str = parts.next().unwrap_or_default();
        let last_str = parts.next().unwrap_or_default();

        let first = first_str.parse::<u64>().unwrap_or_default();
        let last = last_str.parse::<u64>().unwrap_or_default();

        // 2. Iterate through all IDs in the range [first, last]
        for i in first..=last {
            let num_str = i.to_string();
            if num_str.starts_with('0') {
                continue;
            }

            let num_len = num_str.len();
            let mut is_invalid = false;

            // Iterate through all possible pattern lengths (divisors of num_len)
            for pattern_len in 1..=num_len / 2 {
                // The total length must be perfectly divisible by the pattern length
                if num_len % pattern_len == 0 {
                    let first_pattern = &num_str[0..pattern_len];

                    // The number of times the pattern must repeat
                    let num_repetitions = num_len / pattern_len;

                    let mut all_match = true;

                    // Check if all subsequent parts match the first part
                    for r in 1..num_repetitions {
                        let start = r * pattern_len;
                        let end = start + pattern_len;
                        let current_pattern = &num_str[start..end];

                        if first_pattern != current_pattern {
                            all_match = false;
                            break;
                        }
                    }

                    if all_match {
                        is_invalid = true;
                        break;
                    }
                }
            }

            // 3. Add to sum if it's a perfect repetition of N >= 2 times
            if is_invalid {
                invalid_id_sum += i;
            }
        }
    }

    invalid_id_sum
}
