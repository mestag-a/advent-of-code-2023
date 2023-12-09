use std::io;
use std::cmp::min;

fn part1(s: &str) -> u32 {
    s.lines()
    .map(|s| {
        let mut numbers = s
            .chars()
            .filter_map(|c| {
                c.to_digit(10)
            })
            .peekable();

        numbers.peek().unwrap() * 10 + numbers.last().unwrap()
    })
    .sum()
}

const SPELLED_OUT_DIGITS: &[&str] = &["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

fn spelled_out_to_digit(s: &str) -> Option<u32> {
    for (value, &digit) in SPELLED_OUT_DIGITS.iter().enumerate() {
        if digit == &s[..min(s.len(), digit.len())] {
            return Some(value as u32)
        }
    }
    None
}

fn part2(s: &str) -> u32 {
    s.lines()
        .map(|s| -> u32 {
            let mut numbers = s
                .chars()
                .enumerate()
                .filter_map(|(idx, c)| {
                    c.to_digit(10)
                        .or_else(|| { spelled_out_to_digit(&s[idx..]) })
                })
                .peekable();

            numbers.peek().unwrap() * 10 + numbers.last().unwrap()
        })
        .sum()
}

fn main() {
    let s = io::read_to_string(io::stdin()).unwrap();
    let r1 = part1(s.as_str());
    let r2 = part2(s.as_str());

    println!("Part1: {r1}");
    println!("Part2: {r2}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let s = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

        assert_eq!(part1(s), 142);
    }

    #[test]
    fn test_spelled_out_digits() {
        assert_eq!(spelled_out_to_digit("one"), Some(1));
        assert_eq!(spelled_out_to_digit("oneaaaaa"), Some(1));
        assert_eq!(spelled_out_to_digit("eightZZ"), Some(8));
    }

    #[test]
    fn test_part2() {
        let s = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

        assert_eq!(part2(s), 281);
    }
}
