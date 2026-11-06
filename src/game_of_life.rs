// Game of Life, Week 12: grid, display, neighbor counts, the rule for one cell.
//
// TODO: remove the next line once every function below is used.
#![allow(dead_code)]

const ROWS: usize = 10;
const COLS: usize = 20;
type Grid = [[bool; COLS]; ROWS];

// All cells dead.
pub fn new_grid() -> Grid {
    [[false; COLS]; ROWS]
}

// '#' for a live cell, '.' for a dead cell, one line per row.
pub fn print_grid(_grid: &Grid) {
    for row in _grid {
    for &cell in row {
        print!("{}", if cell { '#' } else { '.' });
    }
    println!();
}
}

// Live neighbors of (row, col): the eight surrounding cells, excluding the cell itself.
// Cells beyond the edge count as dead. Careful: row - 1 on a usize of 0 panics.
pub fn count_neighbors(_grid: &Grid, _row: usize, _col: usize) -> u8 {
    let mut count = 0;
for dr in -1isize..=1 {
    for dc in -1isize..=1 {
if dr == 0 && dc == 0 {
    continue;
}
let nr = _row as isize + dr;
let nc = _col as isize + dc;
if nr >= 0
    && nr < ROWS as isize
    && nc >= 0
    && nc < COLS as isize
    && _grid[nr as usize][nc as usize]
{
    count += 1;
}
}
}
count
}

// The rule for one cell: does it live in the next generation?
pub fn next_state(_alive: bool, _neighbors: u8) -> bool {
    _neighbors == 3 || (_alive && _neighbors == 2)
}

// Debug view: each cell's neighbor count as a digit, one line per row.
pub fn print_neighbor_counts(_grid: &Grid) {
    todo!("Implement print_neighbor_counts")
}

// Set a few cells alive (hardcoded, or entered as `row col` pairs until a blank line),
// print the grid, then the neighbor counts.
pub fn play() {
    todo!("Implement play")
}

#[cfg(test)]
mod tests {
    use super::*;

    // Supplied by the instructor.
    #[test]
    fn live_1_dies() {
        assert!(!next_state(true, 1));
    }

    #[test]
    fn live_2_survives() {
        assert!(next_state(true, 2));
    }

    #[test]
    fn dead_3_born() {
        assert!(next_state(false, 3));
    }

    
#[test]
fn live_4_dies() {
    assert!(!next_state(true, 4));
}

#[test]
fn dead_2_stays_dead() {
    assert!(!next_state(false, 2));
}    
}
