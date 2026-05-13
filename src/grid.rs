#![allow(unused_variables)]
#![allow(dead_code)]
use std::collections::HashSet;

// --------Grid--------
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Grid {
    pub cells: HashSet<(i32, i32)>, // Represents the living cells
}

impl Default for Grid {
    fn default() -> Self {
        Self::new()
    }
}

impl Grid {
    pub fn new() -> Self {
        Self {
            cells: HashSet::new(),
        }
    }
    pub fn insert(&mut self, x: i32, y: i32) {
        self.cells.insert((x, y));
    }
}

// Counts the number of living neighbors for a given cell
fn count_neighbors(grid: &Grid, x: i32, y: i32) -> u8 {
    let mut count = 0;
    for dx in -1..=1 {
        for dy in -1..=1 {
            if dx == 0 && dy == 0 {
                continue;
            };

            count += grid.cells.contains(&(x + dx, y + dy)) as u8;
        }
    }

    count
}

fn candidates(grid: &Grid) -> HashSet<(i32, i32)> {
    let mut candidates = HashSet::new();
    for (x, y) in grid.cells.iter() {
        for dx in -1..=1 {
            for dy in -1..=1 {
                candidates.insert((x + dx, y + dy));
            }
        }
    }

    candidates
}

pub fn tick(current: &Grid) -> Grid {
    let mut next = Grid::new();

    for (x, y) in candidates(current) {
        let n = count_neighbors(current, x, y);

        //----Applying rules----
        // adding to new grid only cells that stays or become alive
        let alive = current.cells.contains(&(x, y));
        if alive && (n == 2 || n == 3) || (!alive && n == 3) {
            next.cells.insert((x, y));
        };
    }
    next
}
