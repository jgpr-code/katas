use std::fmt;

// ^ . | - > <

#[derive(Debug)]
struct Pool {
    current_state: Vec<WaterCell>,
    next_state: Vec<WaterCell>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum WaterCell {
    Still,
    Collision,
    Wall,
    Drop,
    Wave(Direction),
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Direction {
    Left,
    Right,
}

impl Pool {
    fn new(number_of_water_cells: usize) -> Self {
        let mut pool = vec![WaterCell::Still; number_of_water_cells + 2];
        if let Some(cell) = pool.first_mut() {
            *cell = WaterCell::Wall;
        };
        if let Some(cell) = pool.last_mut() {
            *cell = WaterCell::Wall;
        };
        Pool {
            current_state: pool.clone(),
            next_state: pool,
        }
    }
}

impl fmt::Display for Pool {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", String::from(self))
    }
}

impl From<&str> for Pool {
    fn from(_: &str) -> Self {
        todo!()
    }
}

impl From<&Pool> for String {
    fn from(pool: &Pool) -> Self {
        pool.current_state.iter().map(|c| char::from(c)).collect()
    }
}

impl fmt::Display for WaterCell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", char::from(self))
    }
}

impl From<char> for WaterCell {
    fn from(cell_char: char) -> Self {
        match cell_char {
            '-' => WaterCell::Still,
            '^' => WaterCell::Collision,
            '|' => WaterCell::Wall,
            '.' => WaterCell::Drop,
            '<' => WaterCell::Wave(Direction::Left),
            '>' => WaterCell::Wave(Direction::Right),

            // right now just assume everything else to be a wall
            _ => WaterCell::Wall,
        }
    }
}

impl From<&WaterCell> for char {
    fn from(water_cell: &WaterCell) -> Self {
        match water_cell {
            WaterCell::Still => '-',
            WaterCell::Collision => '^',
            WaterCell::Wall => '|',
            WaterCell::Drop => '.',
            WaterCell::Wave(direction) => match direction {
                Direction::Left => '<',
                Direction::Right => '>',
            },
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Surrounding {
    left: SurroundingType,
    right: SurroundingType,
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum SurroundingType {
    NotImportant,
    Wave,
    Wall,
}

fn identify_surrounding(
    left_cell: Option<&WaterCell>,
    right_cell: Option<&WaterCell>,
) -> Surrounding {
    Surrounding {
        left: identify_surrounding_in_direction(left_cell, Direction::Left),
        right: identify_surrounding_in_direction(right_cell, Direction::Right),
    }
}

fn identify_surrounding_in_direction(
    cell: Option<&WaterCell>,
    direction: Direction,
) -> SurroundingType {
    match cell {
        // only incoming waves are important
        Some(&WaterCell::Wave(wave_direction)) if direction != wave_direction => {
            SurroundingType::Wave
        }
        // drop is always an incoming wave
        Some(&WaterCell::Drop) => SurroundingType::Wave,
        Some(&WaterCell::Wall) => SurroundingType::Wall,
        _ => SurroundingType::NotImportant,
    }
}

fn next_cell(
    left_cell: Option<&WaterCell>,
    current_cell: &WaterCell,
    right_cell: Option<&WaterCell>,
) -> WaterCell {
    if *current_cell == WaterCell::Wall {
        return WaterCell::Wall;
    }

    let surrounding = identify_surrounding(left_cell, right_cell);

    match current_cell {
        WaterCell::Wave(direction) => handle_wave(&surrounding, direction),
        WaterCell::Drop => handle_drop(&surrounding),
        _ => handle_still_or_collision(&surrounding),
    }
}

fn handle_wave(surrounding: &Surrounding, wave_direction: &Direction) -> WaterCell {
    match *wave_direction {
        Direction::Left => match (surrounding.left, surrounding.right) {
            // |<- to ^
            // |<< to ^
            // |<| to ^
            (SurroundingType::Wall, _) => WaterCell::Collision,
            // -<< to <
            // ><< to <
            (SurroundingType::NotImportant | SurroundingType::Wave, SurroundingType::Wave) => {
                WaterCell::Wave(Direction::Left)
            }
            // -<- to -
            // -<| to -
            // ><- to -
            // ><| to -
            _ => WaterCell::Still,
        },
        Direction::Right => match (surrounding.left, surrounding.right) {
            // >>- to >
            // >>< to >
            (SurroundingType::Wave, SurroundingType::NotImportant | SurroundingType::Wave) => {
                WaterCell::Wave(Direction::Right)
            }
            // ->- to -
            // |>- to -
            (_, SurroundingType::NotImportant) => WaterCell::Still,
            // ->< to ^
            // ->| to ^
            // >>| to ^
            // |>< to ^
            // |>| to ^
            _ => WaterCell::Collision,
        },
    }
}

fn handle_drop(surrounding: &Surrounding) -> WaterCell {
    match (surrounding.left, surrounding.right) {
        // -.- to -
        // >.- to -
        (SurroundingType::NotImportant | SurroundingType::Wave, SurroundingType::NotImportant) => {
            WaterCell::Still
        }
        // -.< to ^
        // -.| to ^
        // >.< to ^
        // >.| to ^
        // |.- to ^
        // |.< to ^
        // |.| to ^
        _ => WaterCell::Collision,
    }
}

fn handle_still_or_collision(surrounding: &Surrounding) -> WaterCell {
    match (surrounding.left, surrounding.right) {
        // >-< to ^
        (SurroundingType::Wave, SurroundingType::Wave) => WaterCell::Collision,
        // --< to <
        // |-< to <
        (_, SurroundingType::Wave) => WaterCell::Wave(Direction::Left),
        // >-| to >
        // >-- to >
        (SurroundingType::Wave, _) => WaterCell::Wave(Direction::Right),
        // --- to -
        // |-| to -
        // --| to -
        // |-- to -
        _ => WaterCell::Still,
    }
}

fn main() {
    let pool = Pool::new(5);
    println!("{:?}", pool);
    println!("{}", pool);
}
