use rayon::prelude::*;
use std::{
    collections::HashSet,
    sync::atomic::{AtomicUsize, Ordering},
};

pub fn main() {
    const INPUT: &str = include_str!("../inputs/6");
    println!(
        "day 6 part 1: {}",
        Map::from_input(INPUT).num_guard_visited_positions()
    );
    println!(
        "day 6 part 2: {}",
        Map::from_input(INPUT).num_new_obstacles_for_loop()
    )
}

// top left is 0,0
// up = north = y-1
// right = east = x+1
#[derive(Debug, Clone)]
struct Map {
    guard: Guard,
    visited: HashSet<Position>,
    visited_direction: HashSet<(Position, Heading)>,
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
        let mut visited_direction = HashSet::new();

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
                    visited_direction.insert((pos, Heading::North));
                }
            }
        }

        Self {
            guard: guard.unwrap_or_default(),
            width,
            height,
            obstructions,
            visited,
            visited_direction,
        }
    }

    fn num_guard_visited_positions(mut self) -> usize {
        self.move_guard_until_off_map().visited.len()
    }

    // brute force :(
    fn num_new_obstacles_for_loop(self) -> usize {
        let success = AtomicUsize::new(0);

        let max_moves = self.height * self.width;
        (0..self.height).par_bridge().for_each(|y| {
            (0..self.width).par_bridge().for_each(|x| {
                let pos = Position { x, y };
                if self.is_obstruction(&pos) || self.guard.position == pos {
                    return;
                }

                let mut temp_map = self.clone();

                temp_map.obstructions.insert(pos.clone());

                for _step in 0..max_moves {
                    let next_pos = temp_map.guard.next_pos();

                    // are we in a loop?
                    if temp_map
                        .visited_direction
                        .contains(&(next_pos.clone(), temp_map.guard.heading))
                    {
                        success.fetch_add(1, Ordering::SeqCst);
                        break;
                    }

                    if temp_map.is_obstruction(&next_pos) {
                        temp_map.guard.turn_right();
                        continue;
                    }
                    if temp_map.is_off_map(&next_pos) {
                        break;
                    }
                    temp_map.guard.move_to(&next_pos);
                    temp_map
                        .visited_direction
                        .insert((next_pos, temp_map.guard.heading));
                }
            })
        });

        success.load(Ordering::Relaxed)
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

#[derive(Debug, Default, Clone, Copy)]
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

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone, Copy)]
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

    #[test]
    fn part_2() {
        let expected = 6;
        let actual = Map::from_input(INPUT).num_new_obstacles_for_loop();
        assert_eq!(expected, actual);
    }
}
