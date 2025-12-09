use std::fs::read_to_string;

fn sum_max_joltage(input: &str, n: usize) -> u64 {
    input
        .lines()
        .map(|line| {
            let batteries = line
                .chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<_>>();
            let mut max_bats = Vec::<u32>::new();
            let mut start_idx = 0;
            for i in 0..n {
                // make sure enough batteries are left after this one
                let max_idx = batteries.len() - (n - i - 1);
                let max_bat = *batteries[start_idx..max_idx].iter().max().unwrap();
                max_bats.push(max_bat);
                // next battery has to follow after previous
                start_idx = batteries[start_idx..max_idx]
                    .iter()
                    .position(|&e| e == max_bat)
                    .unwrap()
                    + 1
                    + start_idx;
            }

            max_bats
                .iter()
                .map(|j| j.to_string())
                .collect::<Vec<_>>()
                .join("")
                .parse::<u64>()
                .unwrap()
        })
        .sum()
}

fn main() {
    let input = read_to_string("../inputs/day3.txt").unwrap_or(String::from(""));

    println!("Sum max joltage (n=2): {}", sum_max_joltage(&input, 2));
    println!("Sum max joltage (n=12): {}", sum_max_joltage(&input, 12));
}
