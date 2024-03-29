mod utils;

use wasm_bindgen::prelude::*;

extern crate web_sys;
use web_sys::console;

// A macro to provide `println!(...)`-style syntax for `console.log` logging.
macro_rules! log {
    ($( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

extern crate fixedbitset;
extern crate js_sys;
use fixedbitset::FixedBitSet;
use std::sync::{Mutex, MutexGuard};

static UNIVERSE: Environment = Environment(Mutex::new(Universe {
    width: 64,
    height: 64,
    cells: FixedBitSet::new(),
}));

#[wasm_bindgen]
pub struct Environment(Mutex<Universe>);

impl Environment {
    pub fn take_universe() -> MutexGuard<'static, Universe> {
        UNIVERSE.0.lock().unwrap()
    }
    pub fn get_index(row: u32, column: u32) -> usize {
        Environment::take_universe().get_index(row, column)
    }
    pub fn get_cells() -> Vec<u32> {
        Environment::take_universe().get_cells().to_owned()
    }
    pub fn set_cells(cells: &[(u32, u32)]) {
        Environment::take_universe().set_cells(cells);
    }
    pub fn set_width(width: u32) {
        Environment::take_universe().set_width(width);
    }
    pub fn set_height(height: u32) {
        Environment::take_universe().set_height(height);
    }
}

#[wasm_bindgen]
impl Environment {
    pub fn build_universe() {
        let size = (Environment::width() * Environment::height()) as usize;
        let mut cells = FixedBitSet::with_capacity(size);
        // 50 % chance of cell being alive on start
        for i in 0..size {
            cells.set(i, js_sys::Math::random() < 0.5);
        }
        Environment::take_universe().cells = cells;
    }

    pub fn tick() {
        Environment::take_universe().tick();
    }

    pub fn width() -> u32 {
        Environment::take_universe().width()
    }
    pub fn height() -> u32 {
        Environment::take_universe().height()
    }

    pub fn toggle_cell(row: u32, column: u32) {
        Environment::take_universe().toggle_cell(row, column)
    }
    pub fn cells() -> *const u32 {
        Environment::take_universe().cells()
    }
}

pub struct Universe {
    width: u32,
    height: u32,
    cells: FixedBitSet,
}

impl Universe {
    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }

    fn get_row_col(&self, idx: usize) -> (u32, u32) {
        let row = idx as u32 / self.width;
        (row, idx as u32 - row)
    }

    fn live_neighbour_count(&self, row: u32, column: u32) -> u8 {
        let mut count = 0;

        let north = if row == 0 { self.height - 1 } else { row - 1 };

        let south = if row == self.height - 1 { 0 } else { row + 1 };

        let west = if column == 0 {
            self.width - 1
        } else {
            column - 1
        };

        let east = if column == self.width - 1 {
            0
        } else {
            column + 1
        };

        let nw = self.get_index(north, west);
        count += self.cells[nw] as u8;

        let n = self.get_index(north, column);
        count += self.cells[n] as u8;

        let ne = self.get_index(north, east);
        count += self.cells[ne] as u8;

        let w = self.get_index(row, west);
        count += self.cells[w] as u8;

        let e = self.get_index(row, east);
        count += self.cells[e] as u8;

        let sw = self.get_index(south, west);
        count += self.cells[sw] as u8;

        let s = self.get_index(south, column);
        count += self.cells[s] as u8;

        let se = self.get_index(south, east);
        count += self.cells[se] as u8;
        count
    }

    /// Get the dead and alive values of the entire universe.
    pub fn get_cells(&self) -> &[u32] {
        &self.cells.as_slice()
    }

    /// Set cells to be alive in a universe by passing the row and column
    /// of each cell as an array.
    pub fn set_cells(&mut self, cells: &[(u32, u32)]) {
        for (row, col) in cells.iter().cloned() {
            let idx = self.get_index(row, col);
            self.cells.set(idx, true);
        }
    }

    pub fn new() -> Universe {
        utils::set_panic_hook();

        let width = 64;
        let height = 64;

        let size = (width * height) as usize;
        let mut cells = FixedBitSet::with_capacity(size);

        // 50 % chance of cell being alive on start
        for i in 0..size {
            cells.set(i, js_sys::Math::random() < 0.5);
        }

        Universe {
            width,
            height,
            cells,
        }
    }

    pub fn render(&self) -> String {
        self.to_string()
    }

    pub fn tick(&mut self) {
        let _timer = Timer::new("Universe::tick");

        let mut next = {
            let _timer = Timer::new("allocate next cells");
            self.cells.clone()
        };

        {
            let _timer = Timer::new("new generation");
            for row in 0..self.height {
                for col in 0..self.width {
                    let idx = self.get_index(row, col);
                    let cell = self.cells[idx];
                    let live_neighbours = self.live_neighbour_count(row, col);

                    /*log!(
                        "cell[{}, {}] is initially {:?} and has {} live neighbours",
                        row,
                        col,
                        cell,
                        live_neighbours
                    );*/

                    next.set(
                        idx,
                        match (cell, live_neighbours) {
                            // Rule 1: Any live cell with fewer than two live neighbours
                            // dies, as if caused by underpopulation.
                            (true, x) if x < 2 => false,
                            // Rule 2:  Any live cell with two or three live neighbours
                            // neighbours dies, as if by overpopulated.
                            (true, 2) | (true, 3) => true,
                            // Rule 3: Any live cell with more than three live
                            // neighbours dies, as if by overpopulation.
                            (true, x) if x > 3 => false,
                            // Rule 4: Any dead cell with exactly three live neighbours
                            // becomes a live cell, as if by reproduction.
                            (false, 3) => true,
                            // All other cells remain in the same state.
                            (otherwise, _) => otherwise,
                        },
                    );

                    //log!("      it becomes {:?}", next[idx]);
                }
            }
        }

        let _timer = Timer::new("free old cells");
        self.cells = next;
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn cells(&self) -> *const u32 {
        self.cells.as_slice().as_ptr()
    }

    /// Set the width of the universe
    ///
    /// Resets all cells to the dead state.
    pub fn set_width(&mut self, width: u32) {
        self.width = width;
        let size = (self.width * self.height) as usize;
        self.cells = FixedBitSet::with_capacity(size);
    }

    /// Set the height of the universe
    ///
    /// Resets all celss to the dead state.
    pub fn set_height(&mut self, height: u32) {
        self.height = height;
        let size = (self.width * self.height) as usize;
        self.cells = FixedBitSet::with_capacity(size);
    }

    pub fn toggle_cell(&mut self, row: u32, column: u32) {
        let idx = self.get_index(row, column);
        let current = self.cells[idx];
        self.cells.set(idx, !current);
    }
}

use std::fmt;

impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.cells.as_slice().chunks(self.width as usize) {
            for &cell in line {
                let symbol = if cell == 0 { '◻' } else { '◼' };
                write!(f, "{}", symbol)?;
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}

pub struct Timer<'a> {
    name: &'a str,
}

impl<'a> Timer<'a> {
    pub fn new(name: &'a str) -> Timer<'a> {
        console::time_with_label(name);
        Timer { name }
    }
}

impl<'a> Drop for Timer<'a> {
    fn drop(&mut self) {
        console::time_end_with_label(self.name);
    }
}
