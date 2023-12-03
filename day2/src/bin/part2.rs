use std::path::Iter;
use std::str::Split;

const MAX_RED: u32 = 12;
const MAX_GREEN: u32 = 13;
const MAX_BLUE: u32 = 14;

struct Round {
    green: u32,
    red: u32,
    blue: u32,
}

pub fn main() {
    let input = include_str!("input1.txt");
    let games: Vec<Vec<&str>> = input.split("\r\n").map(|n| n.split(":").nth(1).unwrap().split(|y| y == ';' || y == ',').collect()).collect();

    let valid: u32 = games.iter().map(|g| match_round((&g).to_vec())).sum();
    println!("{:#?}", valid);
}

fn match_round(numbers: Vec<&str>) -> u32 {
    let mut red = 0;
    let mut green = 0;
    let mut blue = 0;

    for n in numbers.iter() {
        let mut split = n.split(" ");

        let number = split.nth(1).unwrap_or("0").parse::<u32>().unwrap_or(0);
        let colour = split.nth(0).unwrap_or(" ");

        match colour {
            "red" => red = red.max(number),
            "green" => green = green.max(number),
            "blue" => blue = blue.max(number),
            _ => {}
        }
    }

    red * green * blue
}