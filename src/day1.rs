use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

fn main() {
    //read file
    part_one();
    part_two();
}

#[derive(Debug)]
struct NumberMapping {
    map: HashMap<String, i32>,
}

impl NumberMapping {
    fn new() -> Self {
        let mut map = HashMap::new();

        // Add mappings for numbers 1 through 19
        map.insert("one".to_string(), 1);
        map.insert("two".to_string(), 2);
        map.insert("three".to_string(), 3);
        map.insert("four".to_string(), 4);
        map.insert("five".to_string(), 5);
        map.insert("six".to_string(), 6);
        map.insert("seven".to_string(), 7);
        map.insert("eight".to_string(), 8);
        map.insert("nine".to_string(), 9);

        NumberMapping { map }
    }

    fn get_number(&self, word: &str) -> Option<&i32> {
        self.map.get(word)
    }
}

fn part_two() {
    let input = include_str!("../input/day1_1.txt");
    todo!();

}


fn part_one() {
    let file_path = "./input/day1_1.txt";
    let file = File::open(file_path).unwrap();

    // Create a buffered reader to efficiently read the file
    let reader = BufReader::new(file);
    let mut nums: Vec<i32> = Vec::new();
    for line in reader.lines() {
        let parsed = parse_calibration(&line.unwrap());
        if let (Some(first), Some(last)) = (parsed.first(), parsed.last()) {
            let combined_string = first.to_string() + &last.to_string();
            // println!("{}", &combined_string);
            nums.push(combined_string.parse::<i32>().unwrap());
        }
    }

    let sum = nums.iter().fold(0, |acc, &x| acc + x);

    println!("Part 1: {}",sum);
}

fn parse_calibration(input: &str) -> Vec<char> {
    let mut result = Vec::new();
    for char in input.chars() {
        if char.is_digit(10) {
            result.push(char)
        }
    }
    result
}

