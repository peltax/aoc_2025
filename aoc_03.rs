use lazy_static::lazy_static;

lazy_static! {
    pub static ref INPUT: NestedU32 = transform_str_array(&*include_str!("../input/03.txt").lines().collect::<Vec<&str>>()).expect("REASON");

    pub static ref TEST_INPUT: NestedU32 = {
        let test_input: [&str; 4] = [
            "987654321111111",
            "811111111111119",
            "234234234234278",
            "818181911112111",
        ];
        let result = transform_str_array(&test_input);
        result.expect("FATAL: Failed to transform str array.")
    };
}

pub type NestedU32 = Vec<Vec<u32>>;

pub fn transform_str_array(input: &[&str]) -> Result<NestedU32, String> {
    input
        .iter()
        .map(|big_string| {
            big_string
                .chars()
                .map(|c| {
                    c.to_digit(10)
                        .ok_or_else(|| format!("Parsing error: Invalid digit '{}' found.", c))
                })
                .collect::<Result<Vec<u32>, String>>()
        })
        .collect::<Result<NestedU32, String>>()
}

fn find_max_index(arr: &[u32]) -> Option<usize> {
    arr.iter()
        .enumerate()
        .max_by_key(|&(_, &v)| v)
        .map(|(index, _)| index)
}

pub fn solve2(data: &NestedU32, digits: usize) -> u64 {
    let mut result: Vec<u64> = vec![];
    for (line_idx, line) in data.iter().enumerate() {
        let line_length = line.len();
        let mut search_start_idx = 0;
        let mut iteration_selected_digits = Vec::with_capacity(digits);

        for i in 0..digits {
            let digits_left = digits - i;
            let search_end_idx = line_length - (digits_left - 1);
            let window = &line[search_start_idx..search_end_idx];
            let mut max_value = 0;
            let mut max_value_idx = 0;
            for (j, &v) in window.iter().enumerate() {
                if v > max_value {
                    max_value = v;
                    max_value_idx = j;
                }
            }

            let chosen_idx = search_start_idx + max_value_idx;

            iteration_selected_digits.push(line[chosen_idx]);

            println!("line {:?} chosen {:?}", line, line[chosen_idx]);

            search_start_idx = chosen_idx + 1;
        }

        let mut total = 0;
        for &digit in iteration_selected_digits.iter() {
            total = total * 10 + digit as u64;
        }

        result.push(total);

        println!("line {:?} result {:?}", line, result[line_idx]);
    }

    result.iter().fold(0, |acc, x| acc + x)
}

pub fn solve1(data: &NestedU32, digits: usize) -> u64 {
    let mut result: Vec<u64> = vec![];
    for (line_idx, line) in data.iter().enumerate() {
        let line_length = line.len();

        let mut max = 0;
        for i in 0..line_length {
            for j in (i + 1)..line_length {
                let max_candidate = line[i] * 10 + line[j]; // i*tens+ones
                if (max_candidate > max) {
                    max = max_candidate;
                }
            }
        }

        result.push(max as u64);
        println!("line {:?} result {:?}", line, max);
    }

    result.iter().fold(0, |acc, x| acc + x)
}

pub fn part1_solution(data: &NestedU32) -> u64 {
    solve1(data, 2)
}

pub fn part2_solution(data: &NestedU32) -> u64 {
    solve2(data, 12)
}