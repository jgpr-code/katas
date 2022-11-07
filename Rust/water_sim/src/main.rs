use std::fmt;
use std::io::*;
use std::mem;
use std::{thread, time};

// TODO refactor using windows function handling the borders will never be important if they are always Walls...
// borders could also just be waves and model permanently incoming waves

// TODO refactor into multiple files:
// - PoolSimulation.rs
// - Pool.rs
// - WaterCell.rs
// - ?

struct PoolSimulation {
    pool: Pool,
}

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

    fn size(&self) -> usize {
        self.current_state.len()
    }

    fn get_cell(&self, index: isize) -> Option<&WaterCell> {
        if index < 0 || index >= self.size().try_into().unwrap() {
            None
        } else {
            self.current_state.get(index as usize)
        }
    }

    fn next_state(&mut self) {
        for u in 0..self.size() {
            let i = u as isize;
            self.next_state[u] = next_cell(
                self.get_cell(i - 1),
                self.get_cell(i).unwrap(),
                self.get_cell(i + 1),
            );
        }
        mem::swap(&mut self.current_state, &mut self.next_state);
    }

    fn drop_at(&mut self, index: usize) {
        self.current_state[index] = WaterCell::Drop;
    }

    fn get_clear_string(&self) -> String {
        let clearance = vec![0x0008 as u16; self.current_state.len()];
        String::from_utf16(&clearance).expect("clear string construction didn't work as expected")
    }

    fn run_simulation_on_stdio(&mut self) {
        // TODO automatically stop sim after n-steps without change
        let drop_frequency = 5;
        let mut drop_after_steps = drop_frequency;
        loop {
            print!("{}", self);
            std::io::stdout().flush().unwrap();
            self.next_state();
            drop_after_steps -= 1;
            if drop_after_steps == 0 {
                self.drop_at(self.size() / 2);
                drop_after_steps = drop_frequency;
            }
            let sleep_duration = time::Duration::from_millis(500);
            thread::sleep(sleep_duration);
            print!("{}", self.get_clear_string());
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

impl WaterCell {
    fn next_cell(&self, left: &WaterCell, right: &WaterCell) {}
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
    let mut pool = Pool::new(15);
    pool.drop_at(3);
    pool.drop_at(11);
    pool.run_simulation_on_stdio();
}
