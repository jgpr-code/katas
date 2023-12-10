use rand::prelude::*;
use std::fmt;
use std::io::*;
use std::mem;
use std::{thread, time};

#[derive(Debug)]
struct Pool {
    current_state: Vec<PoolCell>,
    next_state: Vec<PoolCell>,
    reflecting_walls: bool,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum PoolCell {
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
    left: SType,
    right: SType,
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum SType {
    Unimportant,
    Wave,
    Wall,
}

struct SimulationConfig {
    iterations: Iterations,
    drop_config: DropConfig,
    update_time: time::Duration,
    enable_reflections: bool,
}
enum Iterations {
    Forever,
    N(u128),
}

struct DropConfig {
    drop_interval: DropInterval,
    drop_location: DropLocation,
}
enum DropInterval {
    None,
    Fixed(u8),
    Range(u8, u8),
}
enum DropLocation {
    None,
    Mid,
    Fixed(usize),
    Random,
}

impl Pool {
    fn new(number_of_water_cells: usize) -> Self {
        let mut pool = vec![PoolCell::Wall];
        pool.append(&mut vec![PoolCell::Still; number_of_water_cells]);
        pool.push(PoolCell::Wall);
        Pool {
            current_state: pool.clone(),
            next_state: pool,
            reflecting_walls: false,
        }
    }

    fn run_simulation_on_stdio(&mut self, config: &SimulationConfig) {
        self.reflecting_walls = config.enable_reflections;
        let clear_string = self.get_clear_string();

        type InitClojure = Box<dyn Fn() -> u8>;
        let init_countdown: InitClojure = match config.drop_config.drop_interval {
            DropInterval::None => Box::new(move || -> u8 { 255 }),
            DropInterval::Fixed(n) => Box::new(move || -> u8 { n }),
            DropInterval::Range(low, high) => {
                Box::new(move || -> u8 { thread_rng().gen_range(low..=high) })
            }
        };

        let mut countdown = init_countdown();

        let mut loop_iter = || {
            countdown -= 1;
            if countdown == 0 {
                self.drop(&config.drop_config.drop_location);
                countdown = init_countdown();
            }
            print!("{}", self);
            std::io::stdout().flush().unwrap();
            self.next_state();
            thread::sleep(config.update_time);
            print!("{}", clear_string);
        };

        match config.iterations {
            Iterations::Forever => loop {
                loop_iter()
            },
            Iterations::N(iterations) => {
                for _ in 0..iterations {
                    loop_iter()
                }
            }
        }
    }

    fn size(&self) -> usize {
        self.current_state.len()
    }

    fn drop(&mut self, config: &DropLocation) {
        let mut rng = thread_rng();
        match config {
            DropLocation::None => (),
            DropLocation::Mid => self.drop_at_mid(),
            DropLocation::Fixed(l) => self.drop_at_any(*l),
            DropLocation::Random => self.drop_at_any(rng.gen_range(1..=self.size() - 2)),
        }
    }

    fn drop_at_any(&mut self, index: usize) {
        self.current_state[index] = PoolCell::Drop;
    }

    fn drop_at_mid(&mut self) {
        self.drop_at_any(self.size() / 2);
    }

    fn drop_at_random(&mut self) {
        let mut rng = thread_rng();
        let location = rng.gen_range(1..=self.size() - 2);
        self.drop_at_any(location);
    }

    fn get_clear_string(&self) -> String {
        // 0x0008 is Backspace
        let clearance = vec![0x0008 as u16; self.current_state.len()];
        String::from_utf16(&clearance).expect("clear string construction didn't work as expected")
    }

    fn next_state(&mut self) {
        for pool_windows in self.current_state.windows(3).enumerate() {
            self.next_state[pool_windows.0 + 1] = self.next_cell(pool_windows.1);
        }
        mem::swap(&mut self.current_state, &mut self.next_state);
    }

    fn next_cell(&self, cells: &[PoolCell]) -> PoolCell {
        if cells.len() != 3 {
            panic!("next_cell needs a slice with excatly 3 cells");
        }
        let left_cell = &cells[0];
        let current_cell = &cells[1];
        let right_cell = &cells[2];

        if *current_cell == PoolCell::Wall {
            return PoolCell::Wall;
        }

        let surrounding = Self::identify_surrounding(left_cell, right_cell);

        match current_cell {
            PoolCell::Wave(direction) => self.handle_wave(&surrounding, direction),
            PoolCell::Drop => self.handle_drop(&surrounding),
            _ => self.handle_still_or_collision(&surrounding),
        }
    }

    fn identify_surrounding(left_cell: &PoolCell, right_cell: &PoolCell) -> Surrounding {
        Surrounding {
            left: Self::identify_surrounding_in_direction(left_cell, Direction::Left),
            right: Self::identify_surrounding_in_direction(right_cell, Direction::Right),
        }
    }

    fn identify_surrounding_in_direction(cell: &PoolCell, direction: Direction) -> SType {
        match cell {
            // only incoming waves are important
            &PoolCell::Wave(wave_direction) if direction != wave_direction => SType::Wave,
            // drop is always an incoming wave
            &PoolCell::Drop => SType::Wave,
            &PoolCell::Wall => SType::Wall,
            _ => SType::Unimportant,
        }
    }

    fn handle_wave(&self, surrounding: &Surrounding, wave_direction: &Direction) -> PoolCell {
        match *wave_direction {
            Direction::Left => match (surrounding.left, surrounding.right) {
                // |<< to ^
                // |<- to ^ or >
                // |<| to ^ or >
                (SType::Wall, s) => {
                    if self.reflecting_walls && s != SType::Wave {
                        PoolCell::Wave(Direction::Right)
                    } else {
                        PoolCell::Collision
                    }
                }
                // -<< to <
                // ><< to <
                (SType::Unimportant | SType::Wave, SType::Wave) => PoolCell::Wave(Direction::Left),
                // -<- to -
                // -<| to -
                // ><- to -
                // ><| to -
                _ => PoolCell::Still,
            },
            Direction::Right => match (surrounding.left, surrounding.right) {
                // >>- to >
                // >>< to >
                (SType::Wave, SType::Unimportant | SType::Wave) => PoolCell::Wave(Direction::Right),
                // ->- to -
                // |>- to -
                (_, SType::Unimportant) => PoolCell::Still,
                // ->| to ^ or <
                // |>| to ^ or <
                (s, SType::Wall) if self.reflecting_walls && s != SType::Wave => {
                    PoolCell::Wave(Direction::Left)
                }
                // ->< to ^
                // >>| to ^
                // |>< to ^
                _ => PoolCell::Collision,
            },
        }
    }

    fn handle_drop(&self, surrounding: &Surrounding) -> PoolCell {
        match (surrounding.left, surrounding.right) {
            // -.- to -
            // >.- to -
            (SType::Unimportant | SType::Wave, SType::Unimportant) => PoolCell::Still,
            // -.| to ^ or <
            // >.| to ^ or <
            (s, SType::Wall) if self.reflecting_walls && s != SType::Wall => {
                PoolCell::Wave(Direction::Left)
            }
            // |.- to ^ or >
            // |.< to ^ or >
            (SType::Wall, s) if self.reflecting_walls && s != SType::Wall => {
                PoolCell::Wave(Direction::Right)
            }
            // -.< to ^
            // >.< to ^
            // |.| to ^
            _ => PoolCell::Collision,
        }
    }

    fn handle_still_or_collision(&self, surrounding: &Surrounding) -> PoolCell {
        match (surrounding.left, surrounding.right) {
            // >-< to ^
            (SType::Wave, SType::Wave) => PoolCell::Collision,
            // --< to <
            // |-< to <
            (_, SType::Wave) => PoolCell::Wave(Direction::Left),
            // >-| to >
            // >-- to >
            (SType::Wave, _) => PoolCell::Wave(Direction::Right),
            // --- to -
            // |-| to -
            // --| to -
            // |-- to -
            _ => PoolCell::Still,
        }
    }
}

impl fmt::Display for Pool {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", String::from(self))
    }
}

impl From<&Pool> for String {
    fn from(pool: &Pool) -> Self {
        pool.current_state.iter().map(|c| char::from(c)).collect()
    }
}

impl From<&str> for Pool {
    fn from(pool_str: &str) -> Self {
        let pool: Vec<PoolCell> = pool_str.chars().map(|c| PoolCell::from(c)).collect();
        Pool {
            current_state: pool.clone(),
            next_state: pool,
            reflecting_walls: false,
        }
    }
}

impl fmt::Display for PoolCell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", char::from(self))
    }
}

impl From<&PoolCell> for char {
    fn from(water_cell: &PoolCell) -> Self {
        use PoolCell::*;
        match water_cell {
            Still => '-',
            PoolCell::Collision => '^',
            PoolCell::Wall => '|',
            PoolCell::Drop => '.',
            PoolCell::Wave(direction) => match direction {
                Direction::Left => '<',
                Direction::Right => '>',
            },
        }
    }
}

impl From<char> for PoolCell {
    fn from(cell_char: char) -> Self {
        use PoolCell::*;
        match cell_char {
            '-' => Still,
            '^' => PoolCell::Collision,
            '|' => PoolCell::Wall,
            '.' => PoolCell::Drop,
            '<' => PoolCell::Wave(Direction::Left),
            '>' => PoolCell::Wave(Direction::Right),

            // right now just assume everything else to be a wall
            _ => PoolCell::Wall,
        }
    }
}

fn main() {
    let config = SimulationConfig {
        iterations: Iterations::N(100),
        drop_config: DropConfig {
            drop_interval: DropInterval::Range(1, 4),
            drop_location: DropLocation::Random,
        },
        update_time: time::Duration::from_millis(500),
        enable_reflections: true,
    };

    let mut pool = Pool::from("|-----.-----------.------------.--------.-------|");
    pool.run_simulation_on_stdio(&config);
}

// TODO write extensive tests

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert!(true);
    }
}
