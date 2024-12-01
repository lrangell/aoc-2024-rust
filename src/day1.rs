use std::collections::HashMap;

use winnow::ascii::digit1;
use winnow::ascii::line_ending;
use winnow::combinator::separated_pair;
use winnow::PResult;
use winnow::Parser;

fn parse_digits(input: &mut &str) -> PResult<(i32, i32)> {
    let p: PResult<(i32, i32)> = separated_pair(digit1, "   ", digit1)
        .map(|(left, right): (&str, &str)| {
            (left.parse::<i32>().unwrap(), right.parse::<i32>().unwrap())
        })
        .parse_next(input);
    let _: PResult<_> = line_ending(input);
    p
}

#[aoc_generator(day1)]
fn parse(input: &str) -> (Vec<i32>, Vec<i32>) {
    let mut input = input;
    let (mut left, mut right): (Vec<i32>, Vec<i32>) = (vec![], vec![]);
    while let Ok((l, r)) = parse_digits(&mut input) {
        left.push(l);
        right.push(r);
    }
    (left, right)
}
#[aoc(day1, part1)]
pub fn part1(input: &(Vec<i32>, Vec<i32>)) -> i32 {
    let (mut left, mut right) = input.to_owned();
    left.sort();
    right.sort();

    left.iter()
        .zip(right.iter())
        .fold(0, |acc, (&l, &r)| acc + (l - r).abs())
}

#[aoc(day1, part2)]
pub fn part2(input: &(Vec<i32>, Vec<i32>)) -> i32 {
    let (left, right) = input.to_owned();
    let mut freq: HashMap<i32, i32> = HashMap::new();

    for n in right {
        *freq.entry(n).or_insert(0) += 1;
    }

    left.iter().map(|n| n * freq.get(n).unwrap_or(&0)).sum()
}

#[cfg(test)]
mod tests {
    use super::{parse, part1, part2};

    #[test]
    fn sample1() {
        let input = parse(
            "3   4
4   3
2   5
1   3
3   9
3   3",
        );
        assert_eq!(part1(&input), 11);
    }
    #[test]
    fn sample2() {
        let input = parse(
            "3   4
4   3
2   5
1   3
3   9
3   3",
        );
        assert_eq!(part2(&input), 31);
    }
}
