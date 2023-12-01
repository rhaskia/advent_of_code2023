fn main() {
    let input = include_str!("input1.txt");

    let numbers: Vec<&str> = vec!["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", ];
    let values = input
        .split("\n")
        .into_iter()
        .map(|line| get_digits(line, &numbers));

    let sum: u32 = values.sum();
    println!("{:?}", sum);
}

fn first_n(s: &str, numbers: &Vec<&str>) -> u32 {
    for i in 0..s.len() {
        let nth_char = s.chars().nth(i).unwrap_or(' ');
        if nth_char.is_digit(10)
        { return nth_char.to_digit(10).unwrap(); }

        for j in 0..10 {
            let n = numbers[j];
            let end = i + n.len();

            if end > s.len() { continue; }

            if s[i..end] == *n { return j as u32; }
        }
    }

    0
}

fn last_n(s: &str, numbers: &Vec<&str>) -> u32 {
    for i in (0..s.len()).rev() {
        let nth_char = s.chars().nth(i).unwrap_or(' ');
        if nth_char.is_digit(10)
        { return nth_char.to_digit(10).unwrap(); }

        for j in 0..10 {
            let n = numbers[j];

            if n.len() > i { continue; }

            if s[i - n.len()..i] == *n {
                return j as u32;
            }
        }
    }

    0
}

fn get_digits(s: &str, numbers: &Vec<&str>) -> u32 {
    return first_n(s, numbers) * 10 + last_n(s, numbers);
}
