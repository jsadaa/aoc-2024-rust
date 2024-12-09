#![allow(clippy::cast_possible_wrap)]
#![allow(clippy::cast_sign_loss)]

use std::str::Lines;

#[derive(Debug)]
struct Grid {
    width: isize,
    height: isize,
    cells: Vec<char>,
}

#[derive(Copy, Clone)]
struct Position {
    x: isize,
    y: isize,
}

impl Position {
    fn new(x: isize, y: isize) -> Self {
        Position { x, y }
    }
}

impl Grid {
    fn new(width: isize, height: isize, cells: Vec<char>) -> Self {
        Grid {
            width,
            height,
            cells,
        }
    }

    fn width(&self) -> isize {
        self.width
    }

    fn height(&self) -> isize {
        self.height
    }

    fn get_index(&self, pos: Position) -> Option<usize> {
        if pos.x >= 0 && pos.x < self.width && pos.y >= 0 && pos.y < self.height {
            Some((pos.y * self.width + pos.x) as usize)
        } else {
            None
        }
    }

    fn get(&self, pos: Position) -> Option<&char> {
        self.get_index(pos).and_then(|index| self.cells.get(index))
    }

    // PART 1

    fn count_matches_from(&self, x: isize, y: isize) -> usize {
        let directions: Vec<[Position; 4]> = vec![
            [
                Position::new(x, y),
                Position::new(x, y - 1),
                Position::new(x, y - 2),
                Position::new(x, y - 3),
            ], // UP
            [
                Position::new(x, y),
                Position::new(x + 1, y - 1),
                Position::new(x + 2, y - 2),
                Position::new(x + 3, y - 3),
            ], // UP RIGHT
            [
                Position::new(x, y),
                Position::new(x + 1, y),
                Position::new(x + 2, y),
                Position::new(x + 3, y),
            ], // RIGHT
            [
                Position::new(x, y),
                Position::new(x + 1, y + 1),
                Position::new(x + 2, y + 2),
                Position::new(x + 3, y + 3),
            ], // DOWN RIGHT
            [
                Position::new(x, y),
                Position::new(x, y + 1),
                Position::new(x, y + 2),
                Position::new(x, y + 3),
            ], // DOWN
            [
                Position::new(x, y),
                Position::new(x - 1, y + 1),
                Position::new(x - 2, y + 2),
                Position::new(x - 3, y + 3),
            ], // DOWN LEFT
            [
                Position::new(x, y),
                Position::new(x - 1, y),
                Position::new(x - 2, y),
                Position::new(x - 3, y),
            ], // LEFT
            [
                Position::new(x, y),
                Position::new(x - 1, y - 1),
                Position::new(x - 2, y - 2),
                Position::new(x - 3, y - 3),
            ], // UP LEFT
        ];

        let mut matches = 0;

        'pos: for positions in &directions {
            let mut string = String::new();

            for &pos in positions {
                if let Some(c) = self.get(pos) {
                    string.push(*c);
                } else {
                    continue 'pos;
                }
            }

            if string == "XMAS" {
                matches += 1;
            }
        }

        matches
    }

    // PART 2

    fn matches_from(&self, x: isize, y: isize) -> bool {
        let Some(up_r) = self.get(Position::new(x + 1, y - 1)) else {
            return false;
        };

        let Some(do_r) = self.get(Position::new(x + 1, y + 1)) else {
            return false;
        };

        let Some(do_l) = self.get(Position::new(x - 1, y + 1)) else {
            return false;
        };

        let Some(up_l) = self.get(Position::new(x - 1, y - 1)) else {
            return false;
        };

        Self::chars_matches(*up_r, *do_l) && Self::chars_matches(*do_r, *up_l)
    }

    fn chars_matches(l: char, r: char) -> bool {
        (l == 'M' && r == 'S') || (l == 'S' && r == 'M')
    }
}

pub(crate) fn day_4_1() {
    let lines = include_str!("../data/day4.txt").lines();

    let width = lines.clone().next().unwrap().len() as isize;
    let height = lines.clone().count() as isize;
    let cells = lines.collect::<Vec<&str>>().join("").chars().collect();

    let grid = Grid::new(width, height, cells);
    let mut matches: usize = 0;

    for y in 0..grid.height() {
        for x in 0..grid.width() {
            let pos = Position::new(x, y);
            if let Some(c) = grid.get(pos) {
                if *c == 'X' {
                    matches += grid.count_matches_from(x, y);
                }
            }
        }
    }

    println!("Number of matches = {matches}");
}

pub(crate) fn day_4_2() {
    let lines = include_str!("../data/day4.txt").lines();
    let width = lines.clone().next().unwrap().len() as isize;
    let height = lines.clone().count() as isize;

    println!("w = {width}, h = {height}");

    let cells = lines.collect::<Vec<&str>>().join("").chars().collect();
    let grid = Grid::new(width, height, cells);
    let mut matches: usize = 0;

    for y in 0..grid.height() {
        for x in 0..grid.width() {
            let pos = Position::new(x, y);
            if let Some(c) = grid.get(pos) {
                if *c == 'A' && grid.matches_from(x, y) {
                    matches += 1;
                }
            }
        }
    }

    println!("Number of matches = {matches}");
}
