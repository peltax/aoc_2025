use lazy_static::lazy_static;

pub const RAW_TEST_INPUT: [&str; 10] = [
    "L68",
    "L30",
    "R48",
    "L5",
    "R60",
    "L55",
    "L1",
    "L99",
    "R14",
    "L82",
];

lazy_static! {
    pub static ref INPUT: Vec<i32> = include_str!("../input/01.txt")
    .lines()
    .map(|line| {
        let s = line.replace('L', "-").replace('R', "");
        s.parse::<i32>().expect("invalid number")
    })
    .collect();

    pub static ref TEST_INPUT: Vec<i32> = RAW_TEST_INPUT.iter()
    .map(|line| {
        let s = line.replace('L', "-").replace('R', "");
        s.parse::<i32>().expect("invalid number")
    })
    .collect();
}

pub fn rotate_part1(data: &[i32]) -> u32 {
    let mut dial = 50;
    let mut zeroes = 0;

    for (&num_cur) in data.iter() {
        dial = (100 + dial + num_cur) % 100;
        if dial == 0 { zeroes += 1; }
    }

    zeroes
}

pub fn rotate_part2(data: &[i32]) -> u32 {
    let mut dial = 50;
    let mut zeroes = 0;

    for (idx, &num_cur) in data.iter().enumerate() {
        let prev = dial;
        let pos_overflow = num_cur.abs() / 100;
        let num =
            if num_cur < 0 { num_cur + pos_overflow * 100 }
            else { num_cur - pos_overflow * 100 };
        let tot = prev + num;

        zeroes += pos_overflow.cast_unsigned();
        dial = (100 + prev + num) % 100;

        if num < 0 && prev != 0 && dial > prevs {
            //println!("underflow idx {idx} zeroes {zeroes} dial {dial} prev {prev} next {num} tot {tot}, zeroes += 1");
            zeroes += 1;
        }

        if tot > 99 {
            //println!("overflow, idx {idx} zeroes {zeroes} dial {dial} prev {prev} next {num} tot {tot}, zeroes += 1");
            zeroes += 1;
        }

        if (tot == 0 && dial == 0) {
            //println!("zero, idx {idx} zeroes {zeroes} dial {dial} prev {prev} next {num} tot {tot}, zeroes += 1");
            zeroes += 1;
        }
    }

    zeroes
}