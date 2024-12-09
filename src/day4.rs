use std::str::Lines;

#[derive(Debug)]
struct Grid {
    width: isize,
    height: isize,
    cells: Vec<char>,
}

impl Grid {
    fn new(width: isize, height: isize, data: Vec<char>) -> Self {
        Grid {
            width,
            height,
            cells: data,
        }
    }

    fn width(&self) -> isize {
        self.width
    }

    fn height(&self) -> isize {
        self.height
    }

    fn get(&self, x: isize, y: isize) -> Option<&char> {
        let index = y * self.width + x;
        if index >= 0 && index < self.cells.len() as isize {
            self.cells.get(index as usize)
        } else {
            None
        }
    }

    fn get_up_from(&self, x: isize, y: isize) -> Option<String> {
        let positions = &[(x, y), (x, y - 1), (x, y - 2), (x, y - 3)];
        let mut string = String::new();

        for (x, y) in positions {
            if let Some(c) = self.get(*x, *y) {
                string.push(*c);
            } else {
                return None;
            }
        }

        if string == "XMAS" {
            Some(string)
        } else {
            None
        }
    }

    fn get_up_right_from(&self, x: isize, y: isize) -> Option<String> {
        let positions = &[(x, y), (x + 1, y - 1), (x + 2, y - 2), (x + 3, y - 3)];
        let mut string = String::new();

        for (x, y) in positions {
            if let Some(c) = self.get(*x, *y) {
                string.push(*c);
            } else {
                return None;
            }
        }

        if string == "XMAS" {
            Some(string)
        } else {
            None
        }
    }

    fn get_right_from(&self, x: isize, y: isize) -> Option<String> {
        let positions = &[(x, y), (x + 1, y), (x + 2, y), (x + 3, y)];
        let mut string = String::new();

        for (x, y) in positions {
            if let Some(c) = self.get(*x, *y) {
                string.push(*c);
            } else {
                return None;
            }
        }

        if string == "XMAS" {
            Some(string)
        } else {
            None
        }
    }

    fn get_lo_right_from(&self, x: isize, y: isize) -> Option<String> {
        let positions = &[(x, y), (x + 1, y + 1), (x + 2, y + 2), (x + 3, y + 3)];
        let mut string = String::new();

        for (x, y) in positions {
            if let Some(c) = self.get(*x, *y) {
                string.push(*c);
            } else {
                return None;
            }
        }

        if string == "XMAS" {
            Some(string)
        } else {
            None
        }
    }

    fn get_lo_from(&self, x: isize, y: isize) -> Option<String> {
        let positions = &[(x, y), (x, y + 1), (x, y + 2), (x, y + 3)];
        let mut string = String::new();

        for (x, y) in positions {
            if let Some(c) = self.get(*x, *y) {
                string.push(*c);
            } else {
                return None;
            }
        }

        if string == "XMAS" {
            Some(string)
        } else {
            None
        }
    }

    fn get_lo_left_from(&self, x: isize, y: isize) -> Option<String> {
        let positions = &[(x, y), (x - 1, y + 1), (x - 2, y + 2), (x - 3, y + 3)];
        let mut string = String::new();

        for (x, y) in positions {
            if let Some(c) = self.get(*x, *y) {
                string.push(*c);
            } else {
                return None;
            }
        }

        if string == "XMAS" {
            Some(string)
        } else {
            None
        }
    }

    fn get_left_from(&self, x: isize, y: isize) -> Option<String> {
        let positions = &[(x, y), (x - 1, y), (x - 2, y), (x - 3, y)];
        let mut string = String::new();

        for (x, y) in positions {
            if let Some(c) = self.get(*x, *y) {
                string.push(*c);
            } else {
                return None;
            }
        }

        if string == "XMAS" {
            Some(string)
        } else {
            None
        }
    }

    fn get_up_left_from(&self, x: isize, y: isize) -> Option<String> {
        let positions = &[(x, y), (x - 1, y - 1), (x - 2, y - 2), (x - 3, y - 3)];
        let mut string = String::new();

        for (x, y) in positions {
            if let Some(c) = self.get(*x, *y) {
                string.push(*c);
            } else {
                return None;
            }
        }

        if string == "XMAS" {
            Some(string)
        } else {
            None
        }
    }

    fn eval_matches_from(&self, x: isize, y: isize) -> usize {
        let matches: Vec<Option<String>> = vec![
            self.get_up_from(x, y),
            self.get_up_right_from(x, y),
            self.get_right_from(x, y),
            self.get_lo_right_from(x, y),
            self.get_lo_from(x, y),
            self.get_lo_left_from(x, y),
            self.get_left_from(x, y),
            self.get_up_left_from(x, y),
        ];

        matches.iter().filter(|m| m.is_some()).count()
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
                matches_count += grid.eval_matches_from(x, y);
            }
        }
    }

    println!("Matches = {matches_count}");
}

pub(crate) fn day_4_2() {}
