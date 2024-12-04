pub fn main() {
    const INPUT: &str = include_str!("../inputs/4");
    println!("day 4 part 1: {}", word_search_total(INPUT, "XMAS"));
}

struct Grid {
    letters: Vec<char>,
    width: usize,
}

impl Grid {
    fn from_input(input: &str) -> Self {
        let width = input.chars().take_while(|c| *c != '\n').count();
        let letters = input.chars().filter(|c| *c != '\n').collect();
        Self { width, letters }
    }

    fn surrounding_words(&self, idx: usize, len: usize) -> [String; 8] {
        let pos = self.position(idx);

        // north
        let north: String = ((pos.row.saturating_sub(len - 1))..=pos.row)
            .rev()
            .filter_map(|row| self.get_xy(pos.column, row))
            .collect();

        // north east
        let north_east: String = ((pos.row.saturating_sub(len - 1))..=pos.row)
            .rev()
            .zip(pos.column..(pos.column + len))
            .filter_map(|(row, column)| self.get_xy(column, row))
            .collect();

        // east
        let east: String = (pos.column..(pos.column + len))
            .filter_map(|column| self.get_xy(column, pos.row))
            .collect();

        // south east
        let south_east: String = (pos.row..(pos.row + len))
            .zip(pos.column..(pos.column + len))
            .filter_map(|(row, column)| self.get_xy(column, row))
            .collect();

        // south
        let south: String = (pos.row..(pos.row + len))
            .filter_map(|row| self.get_xy(pos.column, row))
            .collect();

        // south west
        let south_west: String = (pos.row..(pos.row + len))
            .zip((pos.column.saturating_sub(len - 1)..=pos.column).rev())
            .filter_map(|(row, column)| self.get_xy(column, row))
            .collect();

        // west
        let west: String = (pos.column.saturating_sub(len - 1)..=pos.column)
            .rev()
            .filter_map(|column| self.get_xy(column, pos.row))
            .collect();

        // north west
        let north_west: String = (pos.row.saturating_sub(len - 1)..=pos.row)
            .rev()
            .zip((pos.column.saturating_sub(len - 1)..=pos.column).rev())
            .filter_map(|(row, column)| self.get_xy(column, row))
            .collect();

        [
            north, north_east, east, south_east, south, south_west, west, north_west,
        ]
    }

    fn position(&self, idx: usize) -> Position {
        let row = idx / self.width;
        let column = idx - (row * self.width);
        Position { column, row }
    }

    fn get(&self, pos: &Position) -> Option<char> {
        if pos.column >= self.width {
            return None;
        }
        self.letters.get(pos.row * self.width + pos.column).copied()
    }

    fn get_xy(&self, column: usize, row: usize) -> Option<char> {
        self.get(&Position { column, row })
    }
}

#[derive(Debug)]
struct Position {
    pub column: usize,
    pub row: usize,
}

fn word_search_total(input: &str, needle: &str) -> usize {
    let grid = Grid::from_input(input);
    (0..grid.letters.len())
        .map(|idx| {
            grid.surrounding_words(idx, needle.len())
                .into_iter()
                .filter(|word| word == needle)
                .count()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"#;

    #[test]
    fn part_1() {
        let expected = 18;
        let actual = word_search_total(INPUT, "XMAS");

        assert_eq!(expected, actual);
    }

    #[test]
    fn north() {
        let grid = Grid::from_input(INPUT);
        let expected = "MMSS";
        let actual = &grid.surrounding_words(90, expected.len())[0];

        assert_eq!(expected, actual);
    }

    #[test]
    fn north_east() {
        let grid = Grid::from_input(INPUT);
        let expected = "MAXM";
        let actual = &grid.surrounding_words(90, expected.len())[1];
        assert_eq!(expected, actual);

        let expected = "MMAS";
        let actual = &grid.surrounding_words(30, expected.len())[1];
        assert_eq!(expected, actual);
    }

    #[test]
    fn east() {
        let grid = Grid::from_input(INPUT);
        let expected = "MMMS";
        let actual = &grid.surrounding_words(0, expected.len())[2];
        assert_eq!(expected, actual);

        let expected = "XMAS";
        let actual = &grid.surrounding_words(40, expected.len())[2];
        assert_eq!(expected, actual);
    }

    #[test]
    fn south_east() {
        let grid = Grid::from_input(INPUT);
        let expected = "MSXM";
        let actual = &grid.surrounding_words(0, expected.len())[3];

        assert_eq!(expected, actual);
    }

    #[test]
    fn south() {
        let grid = Grid::from_input(INPUT);
        let expected = "MMAM";
        let actual = &grid.surrounding_words(0, expected.len())[4];

        assert_eq!(expected, actual);
    }

    #[test]
    fn south_west() {
        let grid = Grid::from_input(INPUT);
        let expected = "MSAM";
        let actual = &grid.surrounding_words(9, expected.len())[5];

        assert_eq!(expected, actual);
    }

    #[test]
    fn west() {
        let grid = Grid::from_input(INPUT);
        let expected = "MSAM";
        let actual = &grid.surrounding_words(9, expected.len())[6];
        assert_eq!(expected, actual);

        let expected = "XMAS";
        let actual = &grid.surrounding_words(46, expected.len())[6];
        assert_eq!(expected, actual);
    }

    #[test]
    fn north_west() {
        let grid = Grid::from_input(INPUT);
        let expected = "XMAS";
        let actual = &grid.surrounding_words(99, expected.len())[7];

        assert_eq!(expected, actual);
    }
}