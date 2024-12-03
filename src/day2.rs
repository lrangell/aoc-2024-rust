type Report = Vec<i32>;
type Data = Vec<Report>;

peg::parser! {
  grammar list_parser() for str {
    rule number() -> i32
      = n:$(['0'..='9']+) {? n.parse().or(Err("i32")) }

    rule report() -> Report
      = number() ** " "

    pub rule data() -> Data
          = report() ** "\n"
  }
}

#[aoc_generator(day2)]
pub fn parse(input: &str) -> Data {
    list_parser::data(input).unwrap()
}

#[derive(PartialEq, Debug)]
enum Level {
    Dec,
    Inc,
    Unsafe,
}

impl From<i32> for Level {
    fn from(value: i32) -> Self {
        match value {
            -3..0 => Self::Dec,
            1..=3 => Self::Inc,
            0 => Self::Unsafe,
            _ => Self::Unsafe,
        }
    }
}

fn is_safe(report: &&Report) -> bool {
    let mut iter = report.iter().map_windows(|[&l, &r]| r - l).map(Level::from);

    let level = iter.next().unwrap();
    if level == Level::Unsafe {
        return false;
    }

    iter.all(|l| l == level)
}

#[aoc(day2, part1)]
pub fn part1(input: &Data) -> usize {
    input.iter().filter(is_safe).count()
}

fn is_safe2(report: &Report) -> bool {
    for n in 0..report.len() {
        let mut r = report.clone();
        r.remove(n);
        if is_safe(&&r) {
            return true;
        }
    }
    false
}

#[aoc(day2, part2)]
pub fn part2(input: &Data) -> usize {
    input.iter().filter(|r| is_safe2(r)).count()
}

#[cfg(test)]
mod tests {
    use crate::day2::list_parser;

    use super::{part1, part2};

    fn input() -> &'static str {
        "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"
    }

    #[test]
    fn sample1() {
        let input = list_parser::data(input()).unwrap();
        assert_eq!(part1(&input), 2);
    }

    #[test]
    fn sample2() {
        let input = list_parser::data(input()).unwrap();
        assert_eq!(part2(&input), 4);
    }
}
