fn main() {
    let input = include_str!("input1.txt");

    let numbers: Vec<&str> = vec![
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let values = input
        .split("\n")
        .into_iter()
        .map(|line| get_digits(line, &numbers));

    let sum: u32 = values.sum();
    println!("{:?}", sum);
}

fn first_n(s: &str, numbers: &Vec<&str>) -> u32 {
    for i in 0..s.len() {
        let c = s.chars().nth(i).unwrap_or(' ');
        match c.to_digit(10) {
            Some(d) => return d,
            None => {}
        }

        let value = numbers.iter().enumerate().position(|(_, n)| {
            let end = i + n.len();
            end <= s.len() && s[i..end] == **n
        });

        match value {
            None => {}
            Some(t) => return t as u32,
        };
    }
    0
}

fn last_n(s: &str, numbers: &Vec<&str>) -> u32 {
    for i in (0..s.len()).rev() {
        let c = s.chars().nth(i).unwrap_or(' ');
        match c.to_digit(10) {
            Some(d) => return d,
            None => {}
        }

        let value = numbers
            .iter()
            .enumerate()
            .position(|(_, n)| n.len() <= i && s[i - n.len()..i] == **n);

        match value {
            None => {}
            Some(t) => return t as u32,
        };
    }

    0
}

fn get_digits(s: &str, numbers: &Vec<&str>) -> u32 {
    return first_n(s, numbers) * 10 + last_n(s, numbers);
}
