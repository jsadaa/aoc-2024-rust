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

    fn get_up_pos_from(x: isize, y: isize) -> [Position; 4] {
        [
            Position::new(x, y),
            Position::new(x, y - 1),
            Position::new(x, y - 2),
            Position::new(x, y - 3),
        ]
    }

    fn get_up_right_pos_from(x: isize, y: isize) -> [Position; 4] {
        [
            Position::new(x, y),
            Position::new(x + 1, y - 1),
            Position::new(x + 2, y - 2),
            Position::new(x + 3, y - 3),
        ]
    }

    fn get_right_pos_from(x: isize, y: isize) -> [Position; 4] {
        [
            Position::new(x, y),
            Position::new(x + 1, y),
            Position::new(x + 2, y),
            Position::new(x + 3, y),
        ]
    }

    fn get_lo_right_pos_from(x: isize, y: isize) -> [Position; 4] {
        [
            Position::new(x, y),
            Position::new(x + 1, y + 1),
            Position::new(x + 2, y + 2),
            Position::new(x + 3, y + 3),
        ]
    }

    fn get_lo_pos_from(x: isize, y: isize) -> [Position; 4] {
        [
            Position::new(x, y),
            Position::new(x, y + 1),
            Position::new(x, y + 2),
            Position::new(x, y + 3),
        ]
    }

    fn get_lo_left_pos_from(x: isize, y: isize) -> [Position; 4] {
        [
            Position::new(x, y),
            Position::new(x - 1, y + 1),
            Position::new(x - 2, y + 2),
            Position::new(x - 3, y + 3),
        ]
    }

    fn get_left_pos_from(x: isize, y: isize) -> [Position; 4] {
        [
            Position::new(x, y),
            Position::new(x - 1, y),
            Position::new(x - 2, y),
            Position::new(x - 3, y),
        ]
    }

    fn get_up_left_pos_from(x: isize, y: isize) -> [Position; 4] {
        [
            Position::new(x, y),
            Position::new(x - 1, y - 1),
            Position::new(x - 2, y - 2),
            Position::new(x - 3, y - 3),
        ]
    }

    fn count_matches_from(&self, x: isize, y: isize) -> usize {
        let positions: Vec<[Position; 4]> = vec![
            Self::get_up_pos_from(x, y),
            Self::get_up_right_pos_from(x, y),
            Self::get_right_pos_from(x, y),
            Self::get_lo_right_pos_from(x, y),
            Self::get_lo_pos_from(x, y),
            Self::get_lo_left_pos_from(x, y),
            Self::get_left_pos_from(x, y),
            Self::get_up_left_pos_from(x, y),
        ];

        let mut matches = 0;

        'pos: for pos in &positions {
            let mut string = String::new();

            for &pos in pos {
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
}

pub(crate) fn day_4_1() {
    let lines = include_str!("../data/day4.txt").lines();
    let width = lines.clone().next().unwrap().len() as isize;
    let height = lines.clone().count() as isize;

    println!("w = {width}, h = {height}");

    let cells = lines.collect::<Vec<&str>>().join("").chars().collect();
    let grid = Grid::new(width, height, cells);
    let mut matches_count: usize = 0;

    for y in 0..grid.height() {
        for x in 0..grid.width() {
            let pos = Position::new(x, y);
            if let Some(c) = grid.get(pos) {
                if *c == 'X' {
                    matches_count += grid.count_matches_from(x, y);
                }
            }
        }
    }

    println!("Matches = {matches_count}");
}

pub(crate) fn day_4_2() {}
