use std::collections::HashSet;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Debug, PartialEq, Eq)]
enum Kind {
    Obstruction,
    Empty,
    Visited,
}

enum Event {
    OutOfBound,
    Obstruction,
    InvalidObstruction,
}

#[derive(Debug)]
enum End {
    Full,
    Loop,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
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

    fn rotate(self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}

#[derive(Debug)]
struct Map {
    grid: Vec<Vec<Cell>>,
    current_position: Position,
    current_direction: Direction,
    width: usize,
    height: usize,
    loop_detection: bool,
    visited_states: HashSet<(Position, Direction)>,
}

impl Map {
    fn new(
        grid: Vec<Vec<Cell>>,
        current_position: Position,
        current_direction: Direction,
        loop_detection: bool,
    ) -> Self {
        let width = grid[0].len();
        let height = grid.len();

        let mut visited_states = HashSet::new();
        if loop_detection {
            visited_states.insert((current_position, current_direction));
        }

        Self {
            grid,
            current_position,
            current_direction,
            width,
            height,
            loop_detection,
            visited_states,
        }
    }

    fn next_position(&self) -> Result<Position, Event> {
        let (x, y) = self.current_position.to_tup();

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

    fn next_step(&mut self) -> Result<(), End> {
        let res = self.next_position();

        match res {
            Ok(next_pos) => {
                self.grid[self.current_position.y][self.current_position.x].kind = Kind::Visited;
                self.grid[next_pos.y][next_pos.x].kind = Kind::Visited;

                self.current_position = next_pos;
            }
            Err(Event::Obstruction) => self.current_direction = self.current_direction.rotate(),
            Err(Event::OutOfBound) => return Err(End::Full),
            _ => (),
        }

        if self.loop_detection {
            let state = (self.current_position, self.current_direction);
            if self.visited_states.contains(&state) {
                return Err(End::Loop);
            }

            self.visited_states.insert(state);
        }

        Ok(())
    }

    fn run(&mut self) -> Result<(), End> {
        loop {
            match self.next_step() {
                Ok(()) => continue,
                Err(End::Full) => break,
                Err(End::Loop) => {
                    return Err(End::Loop);
                }
            }
        }

        Ok(())
    }

    fn count_visited_cell(&self) -> usize {
        self.grid
            .iter()
            .map(|v| v.iter().filter(|c| c.kind == Kind::Visited).count())
            .sum()
    }

    fn add_obstruction(&mut self, y: usize, x: usize) -> Result<(), Event> {
        if self.grid[y][x].kind == Kind::Obstruction
            || (self.current_position.x == x && self.current_position.y == y)
        {
            return Err(Event::InvalidObstruction);
        }

        self.grid[y][x].kind = Kind::Obstruction;

        Ok(())
    }

    fn width(&self) -> usize {
        self.width
    }

    fn height(&self) -> usize {
        self.height
    }
}

impl Map {
    fn from_chars(chars: &[Vec<char>], loop_detection: bool) -> Self {
        let mut current_position = Position::new(0, 0);
        let mut current_direction = Direction::Up;

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
                            current_direction = Direction::from(*char);
                            current_position = Position::new(ic, ir);
                            Cell::new(Kind::Visited)
                        }
                        _ => panic!("Invalid char in grid : {char}"),
                    })
                    .collect()
            })
            .collect();

        Map::new(grid, current_position, current_direction, loop_detection)
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

    let mut map = Map::from_chars(&raw_lines, false);
    let _ = map.run();

    let count = map.count_visited_cell();

    println!("Visited count : {count}");
}

pub(crate) fn day_6_2() {
    let raw_lines: Vec<Vec<char>> = include_str!("../data/day6.txt")
        .lines()
        .map(|s| s.chars().collect::<Vec<char>>())
        .collect();

    let height = raw_lines.len();
    let width = raw_lines[0].len();

    let mut loop_counter = 0;

    for row_index in 0..height {
        for col_index in 0..width {
            let mut map = Map::from_chars(&raw_lines, true);

            if map.add_obstruction(row_index, col_index).is_ok() {
                if let Err(End::Loop) = map.run() {
                    loop_counter += 1;
                }
            }
        }
    }

    println!("Loop count : {loop_counter}");
}
