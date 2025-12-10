use std::fs::read_to_string;

#[derive(Copy, Clone, PartialEq)]
enum Tile {
    Empty,
    Paper,
}

impl Tile {
    fn from_char(c: &char) -> Self {
        match c {
            '.' => Tile::Empty,
            '@' => Tile::Paper,
            _ => panic!("Unknown tile {c}"),
        }
    }

    #[allow(dead_code)]
    fn to_str(&self) -> &str {
        match self {
            Tile::Empty => ".",
            Tile::Paper => "@",
        }
    }
}

fn parse_map(input: &str) -> Vec<Vec<Tile>> {
    input
        .lines()
        .map(|line| {
            line.chars().fold(Vec::<Tile>::new(), |mut acc, c| {
                acc.push(Tile::from_char(&c));
                acc
            })
        })
        .collect::<Vec<_>>()
}

fn is_accessible(map: &Vec<Vec<Tile>>, x: usize, y: usize) -> bool {
    // Only paper tiles can be accessible
    if map[y][x] != Tile::Paper {
        return false;
    }

    let mut adjacent_paper = 0;
    for dir_y in -1..=1 {
        for dir_x in -1..=1 {
            if dir_x == 0 && dir_y == 0 {
                continue;
            }

            let new_x = x as isize + dir_x;
            let new_y = y as isize + dir_y;

            if new_x < 0
                || new_y < 0
                || new_x >= map[0].len() as isize
                || new_y >= map.len() as isize
            {
                continue;
            }

            if map[new_y as usize][new_x as usize] == Tile::Paper {
                adjacent_paper += 1;
            }
        }
    }

    adjacent_paper < 4
}

fn accessible_paper_tiles(input: &str) -> usize {
    let map = parse_map(input);

    let mut accessible_count = 0;
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if is_accessible(&map, x, y) {
                accessible_count += 1;
            }
        }
    }

    accessible_count
}

fn paper_count(map: &Vec<Vec<Tile>>) -> usize {
    map.iter()
        .flatten()
        .filter(|&&tile| tile == Tile::Paper)
        .count()
}

fn remove_paper_tiles(map: &mut Vec<Vec<Tile>>) {
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if is_accessible(&map, x, y) {
                map[y][x] = Tile::Empty;
            }
        }
    }
}

fn removable_paper_tiles(input: &str) -> usize {
    let mut map = parse_map(input);
    let initial_paper_count = paper_count(&map);

    // Keep removing accessible paper tiles until no more can be removed
    let mut last_count = initial_paper_count;
    let mut cur_count;
    loop {
        remove_paper_tiles(&mut map);
        cur_count = paper_count(&map);
        if cur_count == last_count {
            break;
        }
        last_count = cur_count;
    }

    initial_paper_count - cur_count
}

fn main() {
    let input = read_to_string("../inputs/day4.txt").unwrap_or(String::from(""));

    println!("Accessible paper tiles: {}", accessible_paper_tiles(&input));
    println!("Removable paper tiles: {}", removable_paper_tiles(&input));
}
