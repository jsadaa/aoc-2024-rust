use std::str::Lines;

#[derive(Debug)]
struct Grid {
    width: usize,
    height: usize,
    cells: Vec<char>,
}

impl Grid {
    fn new(width: usize, height: usize, data: Vec<char>) -> Self {
        Grid {
            width,
            height,
            cells: data,
        }
    }

    fn width(&self) -> usize {
        self.width
    }

    fn height(&self) -> usize {
        self.height
    }

    fn get(&self, x: usize, y: usize) -> Option<&char> {
        if x < self.width && y < self.height {
            self.cells.get(y * self.width + x)
        } else {
            None
        }
    }

    fn get_up(&self, x: usize, y: usize) -> Option<String> {
        if y > 0 {
            let y = y - 1;
            let cache = (0..y)
                .rev()
                .map(|i| self.get(x, i).expect("no cell at {x},{i}"))
                .collect::<Vec<&char>>();

            let string: String = cache.iter().copied().collect();

            if string.len() == 3 {
                Some(string)
            } else {
                None
            }
        } else {
            None
        }
    }

    fn get_up_right(&self, x: usize, y: usize) -> Option<&char> {
        if y > 0 && x < self.width {
            self.get(x + 1, y - 1)
        } else {
            None
        }
    }

    fn get_right(&self, x: usize, y: usize) -> Option<&char> {
        if x < self.width {
            self.get(x + 1, y)
        } else {
            None
        }
    }

    fn get_lo_right(&self, x: usize, y: usize) -> Option<&char> {
        if y < self.height && x < self.width {
            self.get(x + 1, y + 1)
        } else {
            None
        }
    }

    fn get_lo(&self, x: usize, y: usize) -> Option<&char> {
        if y < self.height {
            self.get(x, y + 1)
        } else {
            None
        }
    }

    fn get_lo_left(&self, x: usize, y: usize) -> Option<&char> {
        if y < self.height && x > 0 {
            self.get(x - 1, y + 1)
        } else {
            None
        }
    }

    fn get_left(&self, x: usize, y: usize) -> Option<&char> {
        if x > 0 {
            self.get(x - 1, y)
        } else {
            None
        }
    }

    fn get_up_left(&self, x: usize, y: usize) -> Option<&char> {
        if y > 0 && x > 0 {
            self.get(x - 1, y - 1)
        } else {
            None
        }
    }
}

pub(crate) fn day_4_1() {
    let lines = include_str!("../data/day4.txt").lines();

    let height = 10usize;
    let width = 10usize;

    let cells = lines.collect::<Vec<&str>>().join("").chars().collect();
    let cache: Vec<Vec<(usize, usize)>> = vec![];

    let grid = Grid::new(width, height, cells);

    for y in 0..grid.height() {
        for x in 0..grid.width() {
            let Some(c) = grid.get(x, y) else {
                panic!("No cell at ({x},{y})")
            };
        }
    }
}

pub(crate) fn day_4_2() {}
