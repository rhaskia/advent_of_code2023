fn main() {
    let input = include_str!("input1.txt");
    let values = input.split("\n").into_iter().map(|line| get_digits(line));
    let sum: u32 = values.sum();
    println!("{:?}", sum);
}

fn get_digits(s: &str) -> u32 {
    let first = s
        .chars()
        .find(|n| n.is_digit(10))
        .unwrap_or('0')
        .to_digit(10)
        .unwrap();
    let last = s
        .chars()
        .rev()
        .find(|n| n.is_digit(10))
        .unwrap_or('0')
        .to_digit(10)
        .unwrap();

    return first * 10 + last;
}
