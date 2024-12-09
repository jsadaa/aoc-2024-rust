use std::str::Lines;

#[derive(Debug)]
struct Grid {
    width: isize,
    height: isize,
    cells: Vec<char>,
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

    fn get(&self, x: isize, y: isize) -> Option<&char> {
        if x < 0 || x >= self.width || y < 0 || y >= self.height {
            None
        } else {
            let index = y * self.width + x;
            self.cells.get(index as usize)
        }
    }

    fn get_up_from(x: isize, y: isize) -> [(isize, isize); 4] {
        [(x, y), (x, y - 1), (x, y - 2), (x, y - 3)]
    }

    fn get_up_right_from(x: isize, y: isize) -> [(isize, isize); 4] {
        [(x, y), (x + 1, y - 1), (x + 2, y - 2), (x + 3, y - 3)]
    }

    fn get_right_from(x: isize, y: isize) -> [(isize, isize); 4] {
        [(x, y), (x + 1, y), (x + 2, y), (x + 3, y)]
    }

    fn get_lo_right_from(x: isize, y: isize) -> [(isize, isize); 4] {
        [(x, y), (x + 1, y + 1), (x + 2, y + 2), (x + 3, y + 3)]
    }

    fn get_lo_from(x: isize, y: isize) -> [(isize, isize); 4] {
        [(x, y), (x, y + 1), (x, y + 2), (x, y + 3)]
    }

    fn get_lo_left_from(x: isize, y: isize) -> [(isize, isize); 4] {
        [(x, y), (x - 1, y + 1), (x - 2, y + 2), (x - 3, y + 3)]
    }

    fn get_left_from(x: isize, y: isize) -> [(isize, isize); 4] {
        [(x, y), (x - 1, y), (x - 2, y), (x - 3, y)]
    }

    fn get_up_left_from(x: isize, y: isize) -> [(isize, isize); 4] {
        [(x, y), (x - 1, y - 1), (x - 2, y - 2), (x - 3, y - 3)]
    }

    fn count_matches_from(&self, x: isize, y: isize) -> usize {
        let positions: Vec<[(isize, isize); 4]> = vec![
            Self::get_up_from(x, y),
            Self::get_up_right_from(x, y),
            Self::get_right_from(x, y),
            Self::get_lo_right_from(x, y),
            Self::get_lo_from(x, y),
            Self::get_lo_left_from(x, y),
            Self::get_left_from(x, y),
            Self::get_up_left_from(x, y),
        ];

        let mut matches: Vec<String> = vec![];

        'pos: for pos in &positions {
            let mut string = String::new();

            for (x, y) in pos {
                if let Some(c) = self.get(*x, *y) {
                    string.push(*c);
                } else {
                    continue 'pos;
                }
            }

            if string == "XMAS" {
                matches.push(string);
            }
        }

        matches.len()
    }
}

pub(crate) fn day_4_1() {
    let lines = include_str!("../data/day4.txt").lines();
    let width = lines.clone().take(1).next().unwrap().len();
    let height = lines.clone().count();

    println!("w = {width}, h = {height}");

    let cells = lines.collect::<Vec<&str>>().join("").chars().collect();
    let grid = Grid::new(width as isize, height as isize, cells);
    let mut matches_count: usize = 0;

    for y in 0..grid.height() {
        for x in 0..grid.width() {
            let Some(c) = grid.get(x, y) else {
                panic!("No cell at ({x},{y})")
            };

            if *c == 'X' {
                matches_count += grid.count_matches_from(x, y);
            }
        }
    }

    println!("Matches = {matches_count}");
}

pub(crate) fn day_4_2() {}
