use itertools::Itertools;
use rayon::prelude::*;

pub fn main() {
    const INPUT: &str = include_str!("../inputs/7");
    println!(
        "day 7 part 1: {}",
        total_calibration_result(INPUT, OPS_PART_1)
    );
    println!(
        "day 7 part 2: {}",
        total_calibration_result(INPUT, OPS_PART_2)
    );
}

struct Equation {
    nums: Vec<usize>,
    result: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Operand {
    Add,
    Multiply,
    Concatenate,
}

const OPS_PART_1: [Operand; 2] = [Operand::Add, Operand::Multiply];
const OPS_PART_2: [Operand; 3] = [Operand::Add, Operand::Multiply, Operand::Concatenate];

impl Operand {
    fn apply(&self, a: usize, b: usize) -> usize {
        match self {
            Operand::Add => a + b,
            Operand::Multiply => a * b,
            Operand::Concatenate => format!("{a}{b}").parse().unwrap_or_default(),
        }
    }
}

impl Equation {
    fn has_solution<const N: usize>(&self, ops: [Operand; N]) -> bool {
        let num_operands = self.nums.len() - 1;
        ops.iter()
            .cycle()
            .take(N * num_operands)
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

fn total_calibration_result<const N: usize>(input: &str, ops: [Operand; N]) -> usize {
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
        .filter(|eq| eq.has_solution(ops))
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
        let actual = total_calibration_result(INPUT, OPS_PART_1);
        assert_eq!(expected, actual);
    }

    #[test]
    fn part_2() {
        let expected = 11387;
        let actual = total_calibration_result(INPUT, OPS_PART_2);
        assert_eq!(expected, actual)
    }
}
