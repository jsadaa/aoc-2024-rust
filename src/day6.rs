#[derive(Debug)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

enum Event {
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

    fn next_position(&self) -> Result<Position, Event> {
        let (x, y) = self.current_pos.to_tup();

        let next_y = match self.current_direction {
            Direction::Up => {
                if y == 0 {
                    return Err(Event::OutOfBound);
                }
                y - 1
            }
            Direction::Down => {
                if y >= self.height - 1 {
                    return Err(Event::OutOfBound);
                }
                y + 1
            }
            _ => y,
        };

        let next_x = match self.current_direction {
            Direction::Right => {
                if x >= self.width - 1 {
                    return Err(Event::OutOfBound);
                }
                x + 1
            }
            Direction::Left => {
                if x == 0 {
                    return Err(Event::OutOfBound);
                }
                x - 1
            }
            _ => x,
        };

        match self.grid[next_y][next_x].kind {
            Kind::Obstruction => Err(Event::Obstruction),
            _ => Ok(Position::new(next_x, next_y)),
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
            Err(Event::Obstruction) => self.current_direction = self.current_direction.rotate(),
            Err(Event::OutOfBound) => return Err(()),
        }

        Ok(())
    }

    fn run(mut self) -> Self {
        while self.next_step().is_ok() {}

        self
    }

    fn count_visited_cell(&self) -> usize {
        self.grid
            .iter()
            .map(|v| v.iter().filter(|c| c.kind == Kind::Visited).count())
            .sum()
    }
}

impl Map {
    fn from_chars(chars: &[Vec<char>]) -> Self {
        let mut curr_p = Position::new(0, 0);
        let mut curr_d = Direction::Up;

        let grid = chars
            .iter()
            .enumerate()
            .map(|(ir, row)| {
                row.iter()
                    .enumerate()
                    .map(|(ic, char)| match char {
                        '.' => Cell::new(Kind::Empty),
                        '#' => Cell::new(Kind::Obstruction),
                        '^' | '>' | 'v' | '<' => {
                            curr_d = Direction::from(*char);
                            curr_p = Position::new(ic, ir);
                            Cell::new(Kind::Visited)
                        }
                        _ => panic!("Invalid char in grid : {char}"),
                    })
                    .collect()
            })
            .collect();

        Map::new(grid, curr_p, curr_d)
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
    let raw_lines: Vec<Vec<char>> = include_str!("../data/day6.txt")
        .lines()
        .map(|s| s.chars().collect::<Vec<char>>())
        .collect();

    let map = Map::from_chars(&raw_lines);
    let count = map.run().count_visited_cell();

    println!("Count visited : {count}");
}

pub(crate) fn day_6_2() {}
