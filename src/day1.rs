use std::collections::HashMap;

use rayon::prelude::*;

pub fn main() {
    const INPUT: &str = include_str!("../inputs/1");
    println!("day 1 part 1: {}", total_distance(INPUT));
    println!("day 1 part 2: {}", similarity_score(INPUT));
}

fn parse_input(input: &str) -> (Vec<u32>, Vec<u32>) {
    input
        .par_lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .filter_map(|line| {
            let (left, right) = line.split_once("   ")?;
            let left: u32 = left.parse().ok()?;
            let right: u32 = right.parse().ok()?;
            Some((left, right))
        })
        .unzip()
}

fn total_distance(input: &str) -> u32 {
    let (mut list1, mut list2): (Vec<_>, Vec<_>) = parse_input(input);

    list1.par_sort();
    list2.par_sort();

    list1
        .into_par_iter()
        .zip(list2.into_par_iter())
        .map(|(left, right)| left.abs_diff(right))
        .sum()
}

fn similarity_score(input: &str) -> u32 {
    let (list1, list2): (Vec<_>, Vec<_>) = parse_input(input);

    let right_counts = counts(&list2);

    list1
        .into_par_iter()
        .map(|num| num * right_counts.get(&num).unwrap_or(&0))
        .sum()
}

fn counts(nums: &[u32]) -> HashMap<u32, u32> {
    nums.par_iter()
        .fold(
            || HashMap::<u32, u32>::new(),
            |mut acc, val| {
                acc.entry(*val).and_modify(|e| *e += 1).or_insert(1);
                acc
            },
        )
        .reduce(
            || HashMap::<u32, u32>::new(),
            |mut acc, val| {
                for (k, v) in val {
                    acc.entry(k).and_modify(|e| *e += v).or_insert(v);
                }
                acc
            },
        )
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"3   4
4   3
2   5
1   3
3   9
3   3"#;

    #[test]
    fn part_1() {
        let expected = 11;
        let actual = total_distance(INPUT);

        assert_eq!(expected, actual);
    }

    #[test]
    fn part_2() {
        let expected = 31;
        let actual = similarity_score(INPUT);

        assert_eq!(expected, actual);
    }
}
