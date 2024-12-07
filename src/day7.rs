use itertools::Itertools;
use rayon::prelude::*;

pub fn main() {
    const INPUT: &str = include_str!("../inputs/7");
    println!("day 7 part 1: {}", total_calibration_result(INPUT));
}

struct Equation {
    nums: Vec<usize>,
    result: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Operand {
    Add,
    Multiply,
}

impl Operand {
    fn apply(&self, a: usize, b: usize) -> usize {
        match self {
            Operand::Add => a + b,
            Operand::Multiply => a * b,
        }
    }
}

impl Equation {
    fn has_solution(&self) -> bool {
        const OPS: [Operand; 2] = [Operand::Add, Operand::Multiply];

        let num_operands = self.nums.len() - 1;
        OPS.iter()
            .cycle()
            .take(OPS.len() * num_operands)
            .copied()
            .combinations(num_operands)
            .unique()
            .par_bridge()
            .map(|operands| self.apply_operands(&operands))
            .find_any(|result| *result == self.result)
            .is_some()
    }

    fn apply_operands(&self, operands: &[Operand]) -> usize {
        let mut operands = operands.iter();

        self.nums
            .iter()
            .copied()
            .reduce(|a, b| {
                if a > self.result {
                    return a;
                }
                let Some(op) = operands.next() else {
                    return a;
                };
                op.apply(a, b)
            })
            .unwrap_or_default()
    }
}

fn total_calibration_result(input: &str) -> usize {
    input
        .par_lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .filter_map(|line| line.split_once(": "))
        .filter_map(|(res, nums)| {
            let result = res.parse().ok()?;
            let nums = nums.split(' ').filter_map(|num| num.parse().ok()).collect();

            Some(Equation { nums, result })
        })
        .filter(|eq| eq.has_solution())
        .map(|eq| eq.result)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
"#;

    #[test]
    fn part_1() {
        let expected = 3749;
        let actual = total_calibration_result(INPUT);
        assert_eq!(expected, actual);
    }
}
