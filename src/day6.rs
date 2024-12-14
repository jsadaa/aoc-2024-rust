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
    Loop,
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
    current_pos: Position,
    current_direction: Direction,
    width: usize,
    height: usize,
    loop_detection: bool,
}

impl Map {
    fn new(
        grid: Vec<Vec<Cell>>,
        current_pos: Position,
        current_direction: Direction,
        loop_detection: bool,
    ) -> Self {
        let width = grid[0].len();
        let height = grid.len();
        Self {
            grid,
            current_pos,
            current_direction,
            width,
            height,
            loop_detection,
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
            Kind::Visited => {
                if self.loop_detection {
                    Err(Event::Loop)
                } else {
                    Ok(Position::new(next_x, next_y))
                }
            }
            Kind::Empty => Ok(Position::new(next_x, next_y)),
        }
    }

    fn next_step(&mut self) -> Result<(), End> {
        let res = self.next_position();

        match res {
            Ok(next_pos) => {
                self.grid[self.current_pos.y][self.current_pos.x].kind = Kind::Visited;
                self.grid[next_pos.y][next_pos.x].kind = Kind::Visited;

                self.current_pos = next_pos;
            }
            Err(Event::Obstruction) => self.current_direction = self.current_direction.rotate(),
            Err(Event::OutOfBound) => return Err(End::Full),
            Err(Event::Loop) => return Err(End::Loop),
            _ => (),
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
            || (self.current_pos.x == x && self.current_pos.y == y)
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

        Map::new(grid, curr_p, curr_d, loop_detection)
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
            // Créer une nouvelle carte pour chaque tentative
            let mut map = Map::from_chars(&raw_lines, true);

            // Tenter d'ajouter une obstruction
            if map.add_obstruction(row_index, col_index).is_ok() {
                // Exécuter la simulation
                if let Err(End::Loop) = map.run() {
                    loop_counter += 1;
                }
            }
        }
    }

    println!("Loop count : {loop_counter}");
}
