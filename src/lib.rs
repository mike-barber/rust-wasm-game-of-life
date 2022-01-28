mod utils;

use std::fmt::Display;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// #[wasm_bindgen]
// extern {
//     // imported FROM JS so we can interact with it
//     fn alert(s: &str);
// }

// #[wasm_bindgen]
// pub fn greet(who: &str) {
//     // exported TO JS so this function can be called
//     alert(&format!("Hello, {}, from Rust!", who));
// }

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
}

#[wasm_bindgen]
impl Universe {
    pub fn new() -> Universe {
        let width = 128;
        let height = 128;

        let cells = (0..width * height)
            .map(|_i| {
                // if i % 2 == 0 || i % 7 == 0 {
                //     Cell::Alive
                // } else {
                //     Cell::Dead
                // }
                if js_sys::Math::random() < 0.5 {
                    Cell::Alive
                } else {
                    Cell::Dead
                }
            })
            .collect();

        Universe {
            width,
            height,
            cells,
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }
    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn set_width(&mut self, width: u32) {
        self.width = width;
        self.cells = vec![Cell::Dead; (self.width * self.height) as usize];
    }

    pub fn set_height(&mut self, height: u32) {
        self.height = height;
        self.cells = vec![Cell::Dead; (self.width * self.height) as usize];
    }

    pub fn cells(&self) -> *const Cell {
        self.cells.as_ptr()
    }

    fn get_index(&self, row: u32, col: u32) -> usize {
        (row * self.width + col) as usize
    }

    fn live_neighbour_count(&self, row: u32, col: u32) -> u8 {
        let mut count = 0;
        for drow in [self.height - 1, 0, 1] {
            for dcol in [self.width - 1, 0, 1] {
                if (drow == 0) && (dcol == 0) {
                    continue;
                }
                let r = (row + drow) % self.height;
                let c = (col + dcol) % self.width;
                let ix = self.get_index(r, c);
                count += self.cells[ix] as u8;
            }
        }
        count
    }

    pub fn tick(&mut self) {
        // TODO: flip buffers would be more efficient potentially
        let mut next = self.cells.clone();

        for row in 0..self.height {
            for col in 0..self.width {
                let ix = self.get_index(row, col);

                let this_cell = self.cells[ix];
                let live_neighbours = self.live_neighbour_count(row, col);

                let next_cell = match (this_cell, live_neighbours) {
                    (Cell::Alive, x) if x < 2 => Cell::Dead,
                    (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
                    (Cell::Alive, x) if x > 3 => Cell::Dead,
                    (Cell::Dead, 3) => Cell::Alive,
                    _ => this_cell,
                };

                next[ix] = next_cell;
            }
        }

        self.cells = next;
    }

    // for now, render to a string
    pub fn render(&self) -> String {
        self.to_string()
    }
}

// Note: NOT part of the exposed interface to bindgen, as we only need these
// for testing purposes.
impl Universe {
    pub fn get_cells(&self) -> &[Cell] {
        &self.cells
    }

    pub fn set_cells(&mut self, cell_addresses: &[(u32,u32)]) {
        for (r,c) in cell_addresses.iter().cloned() {
            let ix = self.get_index(r,c);
            self.cells[ix] = Cell::Alive;
        }
    }
}


impl Display for Universe {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for line in self.cells.as_slice().chunks(self.width as usize) {
            for &cell in line {
                write!(
                    f,
                    "{}",
                    match cell {
                        Cell::Dead => '◻',
                        Cell::Alive => '◼',
                    }
                )?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

