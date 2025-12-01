use std::fs::read_to_string;

const DIAL_MAX: i32 = 100;
const DIAL_INITIAL: i32 = 50;

#[derive(PartialEq)]
enum Dir {
    Left,
    Right,
}

fn turn_dial(current: i32, dir: Dir, count: i32) -> (i32, i32) {
    let new = match dir {
        Dir::Left => current - count,
        Dir::Right => current + count,
    };

    // rem = new dial count, div = amount of times we passed 0
    // for negatives moves past 0, div is off by one, unless we are already at 0
    // e.g. from 2 to -5: -5 / 100 = 0 but we passed 0 once
    // on the other hand from 0 to -5: -5 / 100 = 0 but we did not pass 0
    if new <= 0 && current != 0 {
        (new.rem_euclid(DIAL_MAX), (new / DIAL_MAX).abs() + 1)
    } else {
        (new.rem_euclid(DIAL_MAX), (new / DIAL_MAX).abs())
    }
}

fn dial_zero_count(input: &String, f: fn((i32, i32), i32) -> (i32, i32)) -> i32 {
    let dial = input.lines().fold((DIAL_INITIAL, 0), |acc, line| {
        let dir = &line[..1];
        let count = line[1..].parse::<i32>().unwrap();

        let new_dial = match dir {
            "L" => turn_dial(acc.0, Dir::Left, count),
            "R" => turn_dial(acc.0, Dir::Right, count),
            _ => panic!("Invalid line in input: {line}"),
        };

        f(new_dial, acc.1)
    });

    dial.1
}

fn count_zero_at_end(dial: (i32, i32), count: i32) -> (i32, i32) {
    // we don't care how many times we passed 0
    // only how often we hit it after a rotation
    match dial.0 {
        0 => (dial.0, count + 1),
        _ => (dial.0, count),
    }
}

fn count_zero_total(dial: (i32, i32), count: i32) -> (i32, i32) {
    (dial.0, count + dial.1)
}

fn main() {
    let input = read_to_string("../inputs/day1.txt").unwrap_or(String::from(""));

    println!(
        "Dial zero at end: {:?}",
        dial_zero_count(&input, count_zero_at_end)
    );
    println!(
        "Dial zero total: {:?}",
        dial_zero_count(&input, count_zero_total)
    );
}
