use rayon::prelude::*;

pub fn main() {
    const INPUT: &str = include_str!("../inputs/2");
    println!("day 2 part 1: {}", num_safe_reports(INPUT));
}

fn num_safe_reports(input: &str) -> usize {
    parse_input(input)
        .into_iter()
        .filter(|report| is_safe(report))
        .count()
}

fn is_safe(levels: &[usize]) -> bool {
    let all_increasing = || {
        levels.windows(2).all(|vals| {
            let [a, b, ..] = vals else {
                return false;
            };
            b > a
        })
    };
    let all_decreasing = || {
        levels.windows(2).all(|vals| {
            let [a, b, ..] = vals else {
                return false;
            };
            b < a
        })
    };

    let mut diffs = levels.windows(2).map(|vals| {
        let [a, b] = vals else {
            return 0;
        };
        a.abs_diff(*b)
    });

    (all_increasing() || all_decreasing()) && diffs.all(|diff| diff >= 1 && diff <= 3)
}

fn parse_input(input: &str) -> Vec<Vec<usize>> {
    input
        .par_lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(|line| {
            let levels = line
                .split(' ')
                .filter_map(|num| num.parse().ok())
                .collect::<Vec<usize>>();
            levels
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"#;

    #[test]
    fn part_1() {
        let expected = 2;
        let actual = num_safe_reports(INPUT);

        assert_eq!(expected, actual);
    }
}
