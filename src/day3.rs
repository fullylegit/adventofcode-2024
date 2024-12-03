use nom::bytes::complete::tag;
use nom::character::complete::{digit0, digit1};
use nom::combinator::{map_res, recognize};
use nom::sequence::tuple;
use nom::IResult;

pub fn main() {
    const INPUT: &str = include_str!("../inputs/3");
    println!("day 3 part 1: {}", uncorrupted_mul_sum(INPUT));
    println!("day 3 part 2: {}", uncorrupted_conditional_mul_sum(INPUT));
}

fn uncorrupted_mul_sum(input: &str) -> usize {
    let mut total = 0;

    let mut remaining = Some(input);
    while let Some(input) = remaining {
        match mul(input) {
            Ok((input, (num1, num2))) => {
                total += num1 * num2;
                remaining = Some(input);
            }
            Err(_) => remaining = input.get(1..),
        }
    }

    total
}

fn uncorrupted_conditional_mul_sum(input: &str) -> usize {
    let mut total = 0;

    let mut mul_state = State::Enabled;
    let mut remaining = Some(input);
    while let Some(input) = remaining {
        match mul_state {
            State::Enabled => {
                if let Ok((input, (num1, num2))) = mul(input) {
                    remaining = Some(input);
                    total += num1 * num2;
                    continue;
                }
                if let Ok((input, ())) = disable(input) {
                    remaining = Some(input);
                    mul_state = State::Disabled;
                    continue;
                }
                remaining = input.get(1..);
            }
            State::Disabled => match enable(input) {
                Ok((input, ())) => {
                    remaining = Some(input);
                    mul_state = State::Enabled;
                }
                Err(_) => remaining = input.get(1..),
            },
        }
    }

    total
}

fn mul(input: &str) -> IResult<&str, (usize, usize)> {
    let (input, _) = tag("mul(")(input)?;
    let (input, num1) = number(input)?;
    let (input, _) = tag(",")(input)?;
    let (input, num2) = number(input)?;
    let (input, _) = tag(")")(input)?;

    Ok((input, (num1, num2)))
}

fn number(input: &str) -> IResult<&str, usize> {
    map_res(recognize(tuple((digit1, digit0, digit0))), |s: &str| {
        s.parse()
    })(input)
}

fn enable(input: &str) -> IResult<&str, ()> {
    let (remaining, _) = tag("do()")(input)?;
    Ok((remaining, ()))
}

fn disable(input: &str) -> IResult<&str, ()> {
    let (remaining, _) = tag("don't()")(input)?;
    Ok((remaining, ()))
}

enum State {
    Enabled,
    Disabled,
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT1: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    const INPUT2: &str =
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    #[test]
    fn part_1() {
        let expected = 161;
        let actual = uncorrupted_mul_sum(INPUT1);

        assert_eq!(expected, actual);
    }

    #[test]
    fn part_2() {
        let expected = 48;
        let actual = uncorrupted_conditional_mul_sum(INPUT2);

        assert_eq!(expected, actual);
    }
}
