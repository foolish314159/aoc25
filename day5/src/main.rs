use std::{fs::read_to_string, ops::RangeInclusive};

struct Database {
    fresh: Vec<RangeInclusive<u64>>,
    available: Vec<u64>,
}

fn parse_database(input: &str) -> Database {
    let mut parts = input.split("\n\n");

    let fresh = parts.next().unwrap().lines().fold(vec![], |mut acc, line| {
        let range = line
            .split("-")
            .map(|n| n.parse::<u64>().unwrap())
            .collect::<Vec<_>>();
        acc.push(range[0]..=range[1]);
        acc
    });

    let available = parts
        .next()
        .unwrap()
        .lines()
        .map(|line| line.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    Database { fresh, available }
}

fn fresh_available_ingredients(input: &str) -> usize {
    let db = parse_database(input);

    db.available
        .iter()
        .filter(|&id| db.fresh.iter().any(|range| range.contains(id)))
        .count()
}

fn merge_ranges(ranges: &Vec<RangeInclusive<u64>>) -> Vec<RangeInclusive<u64>> {
    let mut sorted = ranges.clone();
    sorted.sort_by_key(|r| *r.start());

    let mut merged = vec![];
    let mut prev_start = *sorted[0].start();
    let mut prev_end = *sorted[0].end();

    for range in &sorted[1..] {
        let current_start = *range.start();
        let current_end = *range.end();

        if current_start <= prev_end {
            // Overlapping ranges, merge them
            prev_end = prev_end.max(current_end);
        } else {
            // Non overlapping ranges, add previous and restart with current
            merged.push(prev_start..=prev_end);
            prev_start = current_start;
            prev_end = current_end;
        }
    }

    merged.push(prev_start..=prev_end);
    merged
}

fn fresh_ingredients(input: &str) -> u64 {
    let fresh = parse_database(input).fresh;
    let merged = merge_ranges(&fresh);

    merged
        .iter()
        .map(|range| range.clone().count() as u64)
        .sum()
}

fn main() {
    let input = read_to_string("../inputs/day5.txt").unwrap_or(String::from(""));

    println!(
        "Fresh available ingredients: {}",
        fresh_available_ingredients(&input)
    );
    println!("Fresh ingredients: {}", fresh_ingredients(&input));
}
