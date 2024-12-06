use std::collections::HashSet;

pub fn main() {
    const INPUT: &str = include_str!("../inputs/6");
    println!(
        "day 6 part 1: {}",
        Map::from_input(INPUT).num_guard_visited_positions()
    );
}

// top left is 0,0
// up = north = y-1
// right = east = x+1
struct Map {
    guard: Guard,
    visited: HashSet<Position>,
    obstructions: HashSet<Position>,
    width: isize,
    height: isize,
}

impl Map {
    fn from_input(input: &str) -> Self {
        const OBSTRUCTION: char = '#';
        const GUARD: char = '^';

        let mut lines = input.lines().map(|line| line.trim()).peekable();

        let width = lines.peek().map(|s| *s).unwrap_or_default().len() as isize;
        let height = input.len() as isize / (width + 1);

        let mut obstructions = HashSet::new();
        let mut guard = None;
        let mut visited = HashSet::new();

        for (y, line) in lines.enumerate() {
            for (x, c) in line.char_indices() {
                let pos = Position {
                    x: x as _,
                    y: y as _,
                };
                if c == OBSTRUCTION {
                    obstructions.insert(pos);
                } else if c == GUARD {
                    guard = Some(Guard::new(pos.clone(), Heading::North));
                    visited.insert(pos);
                }
            }
        }

        Self {
            guard: guard.unwrap_or_default(),
            width,
            height,
            obstructions,
            visited,
        }
    }

    fn num_guard_visited_positions(mut self) -> usize {
        self.move_guard_until_off_map().visited.len()
    }

    fn move_guard_until_off_map(&mut self) -> &mut Self {
        loop {
            let next_pos = self.guard.next_pos();
            if self.is_obstruction(&next_pos) {
                self.guard.turn_right();
                continue;
            }
            if self.is_off_map(&next_pos) {
                break;
            }
            self.guard.move_to(&next_pos);
            self.visited.insert(next_pos);
        }

        self
    }

    fn is_obstruction(&self, pos: &Position) -> bool {
        self.obstructions.contains(pos)
    }

    fn is_off_map(&self, pos: &Position) -> bool {
        pos.x < 0 || pos.y < 0 || pos.x >= self.width || pos.y >= self.height
    }
}

#[derive(Default)]
struct Guard {
    position: Position,
    heading: Heading,
}

impl Guard {
    fn new(position: Position, heading: Heading) -> Self {
        Self { position, heading }
    }

    fn next_pos(&self) -> Position {
        match self.heading {
            Heading::North => Position {
                x: self.position.x,
                y: self.position.y - 1,
            },
            Heading::East => Position {
                x: self.position.x + 1,
                y: self.position.y,
            },
            Heading::South => Position {
                x: self.position.x,
                y: self.position.y + 1,
            },
            Heading::West => Position {
                x: self.position.x - 1,
                y: self.position.y,
            },
        }
    }

    fn turn_right(&mut self) {
        self.heading.turn_right();
    }

    fn move_to(&mut self, next_pos: &Position) {
        self.position = *next_pos;
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Default)]
struct Position {
    x: isize,
    y: isize,
}

#[derive(Default)]
enum Heading {
    #[default]
    North,
    East,
    South,
    West,
}

impl Heading {
    fn turn_right(&mut self) -> &mut Self {
        match self {
            Heading::North => *self = Heading::East,
            Heading::East => *self = Heading::South,
            Heading::South => *self = Heading::West,
            Heading::West => *self = Heading::North,
        }
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
"#;

    #[test]
    fn part_1() {
        let expected = 41;
        let actual = Map::from_input(INPUT).num_guard_visited_positions();
        assert_eq!(expected, actual);
    }
}
