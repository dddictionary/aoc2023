fn main() {
    part_one();
}

struct Game {
    id: u32,
    blue: u32,
    red: u32,
    green: u32
}

fn part_one() {
    const MAX_RED: u32 = 12;
    const MAX_GREEN: u32 = 13;
    const MAX_BLUE: u32 = 14;

    let input = include_str!("../input/day2_1.txt");
    parse_input(input);
}

fn parse_input(input: &str) {
    let segments: Vec<Vec<&str>> = input.lines().map(|line| {
        line.split(";").collect()
    }).collect();

    println!("{:?}", segments);
}