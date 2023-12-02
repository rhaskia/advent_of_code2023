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
    let games = input.split("\r\n").map(|n| n.split(":").nth(1).unwrap());
    let rounds = games.map(|round| round.split(";").collect::<Vec<&str>>()).collect::<Vec<Vec<&str>>>();

    let valid = rounds
        .iter().enumerate().map(|r|
        (r.0, r.1.iter().map(|n| match_round(n)).all(|m|m)))//
        .filter(|n| n.1);
    println!("{:#?}", valid.fold(0, |acc, num| acc + num.0 + 1));
}

fn match_round(s: &str) -> bool {
    let numbers:Vec<&str> = s.split(",").collect();
    let mut red = 0;
    let mut green = 0;
    let mut blue = 0;

    for n in numbers.iter() {
        let mut split = n.split(" ");

        let number = split.nth(1).unwrap_or("0").parse::<u32>().unwrap_or(0);
        let colour = split.nth(0).unwrap_or(" ");

        match colour {
            "red" => red += number,
            "green" => green += number,
            "blue" => blue += number,
            _ => {}
        }

        if red > MAX_RED || green > MAX_GREEN || blue > MAX_BLUE { return false; }
    }

    true
}