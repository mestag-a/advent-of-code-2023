use std::io;

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

fn part2(s: &str) -> u32 {
    0
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
}
