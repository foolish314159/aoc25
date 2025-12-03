use std::fs::read_to_string;

// for part 1 an id is invalid if the left half is the same as the right half
fn is_invalid_part1(id: u64) -> bool {
    let id_str = id.to_string();
    let len = id_str.chars().count();

    // uneven length id can never be invalid, because it can't be split in half evenly
    if len % 2 != 0 {
        return false;
    }

    let left = &id_str[..len / 2];
    let right = &id_str[len / 2..];

    left == right
}

// for part 2 an id is invalid if any sequence of digits repeats at least twice
fn is_invalid_part2(id: u64) -> bool {
    let id_str = id.to_string();
    let len = id_str.chars().count();

    // for a sequence of digits to be possible to repeat the whole id's
    // length must be divisible by the sequence's length
    // e.g. in a 8 digit id, only sequences of length 1, 2 or 4 can repeat
    // so if there is a remainder when dividing it can't be invalid
    // the max sequence length is therefore len/2
    for seq_len in 1..=(len / 2) {
        if len % seq_len != 0 {
            continue;
        }

        let n_parts = len / seq_len;
        let mut parts = Vec::<&str>::new();
        for i in 0..n_parts {
            let start = i * seq_len;
            let end = start + seq_len;
            let part = &id_str[start..end];
            parts.push(part);
        }

        // sequence repeats => id is invalid
        if parts.iter().all(|&part| part == parts[0]) {
            return true;
        }
    }

    false
}

fn sum_of_invalid_ids(input: &String, f_is_invalid: fn(u64) -> bool) -> u64 {
    let invalid_ids = input.split(",").fold(Vec::<u64>::new(), |mut acc, range| {
        let parts = range
            .split("-")
            .map(|id| id.parse::<u64>().unwrap())
            .collect::<Vec<_>>();

        for id in parts[0]..=parts[1] {
            if f_is_invalid(id) {
                acc.push(id);
            }
        }

        acc
    });

    invalid_ids.iter().sum()
}

fn main() {
    let input = read_to_string("../inputs/day2.txt").unwrap_or(String::from(""));

    println!(
        "Sum of invalid IDs part1: {}",
        sum_of_invalid_ids(&input, is_invalid_part1)
    );

    println!(
        "Sum of invalid IDs part2: {}",
        sum_of_invalid_ids(&input, is_invalid_part2)
    );
}
