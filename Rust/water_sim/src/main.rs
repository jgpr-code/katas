// droplet rules
// -.- to <->
// --- to ---
// <-- to

// 5 different things | ^ - < > 3 locations -> 5^3 = 125 combinations => not feasible

// -< to <-

// . basicall is < and >
// each cell has < and > if both meet and not . then ^
// | is basically a constant >< that doesn't propagate -> special case for first and last cell

use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum WaterCell {
    Still,
    Collision,
    Wall,
    Drop,
    LeftWave,
    RightWave,
}

impl fmt::Display for WaterCell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Still => "-",
                Collision => "^",
                Wall => "|",
                Drop => ".",
                LeftWave => "<",
                RightWave => ">",
            }
        )
    }
}

enum SourroundingType {
    Wall,
    Wave,
}

enum Surrounding {
    Left(SourroundingType),
    Right(SourroundingType),
}

fn next_cell(
    left_cell: Option<WaterCell>,
    current_cell: WaterCell,
    right_cell: Option<WaterCell>,
) -> WaterCell {
    let wave_from_left =
        left_cell.map_or(false, |c| c == WaterCell::LeftWave || c == WaterCell::Drop);
    let wave_from_right =
        right_cell.map_or(false, |c| c == WaterCell::RightWave || c == WaterCell::Drop);
    let wall_at_left = left_cell.map_or(false, |c| c == WaterCell::Wall);
    let wall_at_right = right_cell.map_or(false, |c| c == WaterCell::Wall);

    match (
        wall_at_left,
        wave_from_left,
        current_cell,
        wave_from_right,
        wall_at_right,
    ) {
        (_, _, WaterCell::Wall, _, _) => WaterCell::Wall,
        (_, _, WaterCell::Drop, _, _) => current_cell,
        (_, _, WaterCell::LeftWave, _, _) => current_cell,
        (_, _, WaterCell::RightWave, _, _) => current_cell,
        (_, _, WaterCell::Still, _, _) => current_cell,
        (_, _, WaterCell::Collision, _, _) => current_cell,
    }
}

fn simulate_step(current_state: &mut Vec<WaterCell>, next_state: &mut Vec<WaterCell>) {
    next_state = current_state;
    for i in 0..next_state.len() {
        next_state[i] = next_cell(next_state.get(i - 1), next_state[i], next_state.get(i + 1));
    }
    current_state = next_state;
}

fn main() {
    let mut current_state = vec![WaterCell::LeftWave];
    let mut next_state = vec![WaterCell::RightWave];

    simulate_step(&mut current_state, &mut next_state);
}
