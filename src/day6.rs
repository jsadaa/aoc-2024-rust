#[derive(Debug)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

enum Error {
    OutOfBound,
    Obstruction,
}

impl Direction {
    fn from(char: char) -> Self {
        match char {
            '^' => Direction::Up,
            '>' => Direction::Right,
            'v' => Direction::Down,
            '<' => Direction::Left,
            _ => panic!("Invalid direction : {char}"),
        }
    }

    fn rotate(&self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Kind {
    Obstruction,
    Empty,
    Visited,
}

#[derive(Clone, Copy, Debug)]
struct Position {
    x: usize,
    y: usize,
}

impl Position {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    fn to_tup(self) -> (usize, usize) {
        (self.x, self.y)
    }
}

#[derive(Debug)]
struct Map {
    grid: Vec<Vec<Cell>>,
    current_pos: Position,
    current_direction: Direction,
    width: usize,
    height: usize,
}

impl Map {
    fn new(grid: Vec<Vec<Cell>>, current_pos: Position, current_direction: Direction) -> Self {
        let width = grid[0].len();
        let height = grid.len();
        Self {
            grid,
            current_pos,
            current_direction,
            width,
            height,
        }
    }

    fn next_position(&self) -> Result<Position, Error> {
        let (x, y) = self.current_pos.to_tup();

        let y: isize = match self.current_direction {
            Direction::Up => y as isize - 1,
            Direction::Down => y as isize + 1,
            _ => y as isize,
        };

        let x: isize = match self.current_direction {
            Direction::Right => x as isize + 1,
            Direction::Left => x as isize - 1,
            _ => x as isize,
        };

        if y < 0 || y >= self.height as isize || x < 0 || x >= self.width as isize {
            return Err(Error::OutOfBound);
        }

        match self
            .grid
            .get(y as usize)
            .unwrap()
            .get(x as usize)
            .unwrap()
            .kind
        {
            Kind::Obstruction => Err(Error::Obstruction),
            _ => Ok(Position::new(x as usize, y as usize)),
        }
    }

    fn next_step(&mut self) -> Result<(), ()> {
        let res = self.next_position();

        match res {
            Ok(next_pos) => {
                self.grid[self.current_pos.y][self.current_pos.x].kind = Kind::Visited;
                self.grid[next_pos.y][next_pos.x].kind = Kind::Visited;

                self.current_pos = next_pos;
            }
            Err(Error::Obstruction) => self.current_direction = self.current_direction.rotate(),
            Err(Error::OutOfBound) => return Err(()),
        }

        Ok(())
    }

    fn run(mut self) -> usize {
        while self.next_step().is_ok() {}

        self.grid
            .iter()
            .map(|v| v.iter().filter(|c| c.kind == Kind::Visited).count())
            .sum()
    }
}

#[derive(Debug)]
struct Cell {
    kind: Kind,
}

impl Cell {
    fn new(kind: Kind) -> Self {
        Self { kind }
    }
}

pub(crate) fn day_6_1() {
    let lines: Vec<Vec<char>> = include_str!("../data/day6.txt")
        .lines()
        .map(|s| s.chars().collect::<Vec<char>>())
        .collect();

    let mut grid: Vec<Vec<Cell>> = vec![];
    let mut curr_p = Position::new(0, 0);
    let mut curr_d = Direction::Up;

    for (il, line) in lines.iter().enumerate() {
        let mut row: Vec<Cell> = vec![];
        for (ic, char) in line.iter().enumerate() {
            let cell = match char {
                '.' => Cell::new(Kind::Empty),
                '#' => Cell::new(Kind::Obstruction),
                '^' | '>' | 'v' | '<' => {
                    curr_d = Direction::from(*char);
                    curr_p = Position::new(ic, il);
                    Cell::new(Kind::Visited)
                }
                _ => panic!("Invalid char in grid : {char}"),
            };
            row.push(cell);
        }
        grid.push(row);
    }

    let map = Map::new(grid, curr_p, curr_d);

    let count = map.run();

    println!("Count visited : {count}");
}

pub(crate) fn day_6_2() {}
