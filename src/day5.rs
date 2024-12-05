pub fn main() {
    const INPUT: &str = include_str!("../inputs/5");
    println!("day 5 part 1: {}", correct_order_middle_page_total(INPUT));
}

struct Update {
    pages: Vec<usize>,
}

struct Rule {
    before: usize,
    after: usize,
}

impl Update {
    fn satisfies_rule(&self, rule: &Rule) -> bool {
        let Some(before_page_idx) = self.index_of_page(rule.before) else {
            return true;
        };
        let Some(after_page_idx) = self.index_of_page(rule.after) else {
            return true;
        };
        before_page_idx < after_page_idx
    }

    fn index_of_page(&self, idx: usize) -> Option<usize> {
        self.pages
            .iter()
            .enumerate()
            .find(|(_idx, page)| **page == idx)
            .map(|(idx, _page)| idx)
    }

    fn middle_page(&self) -> usize {
        self.pages[self.pages.len() / 2]
    }
}

impl FromIterator<usize> for Update {
    fn from_iter<T: IntoIterator<Item = usize>>(iter: T) -> Self {
        Self {
            pages: iter.into_iter().collect(),
        }
    }
}

fn correct_order_middle_page_total(input: &str) -> usize {
    let (rules, updates) = parse_input(input);

    updates
        .into_iter()
        .filter_map(|update| {
            if rules.iter().all(|rule| update.satisfies_rule(rule)) {
                Some(update.middle_page())
            } else {
                None
            }
        })
        .sum()
}

fn parse_input(input: &str) -> (Vec<Rule>, Vec<Update>) {
    let lines = input
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty());

    let (rules, updates): (Vec<&str>, Vec<&str>) = lines.partition(|line| is_rule(line));

    let rules = rules
        .into_iter()
        .filter_map(|line| {
            let (before, after) = line.split_once('|')?;
            let before = before.parse().ok()?;
            let after = after.parse().ok()?;
            Some(Rule { before, after })
        })
        .collect();

    let updates = updates
        .into_iter()
        .map(|line| line.split(',').filter_map(|num| num.parse().ok()).collect())
        .collect();

    (rules, updates)
}

fn is_rule(line: &str) -> bool {
    line.contains('|')
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
"#;

    #[test]
    fn part_1() {
        let expected = 143;
        let actual = correct_order_middle_page_total(INPUT);
        assert_eq!(expected, actual);
    }
}
