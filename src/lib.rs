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

impl Universe {
    pub fn new() -> Universe {
        let width = 64;
        let height = 64;

        let cells = (0..width * height)
            .map(|i| {
                if i % 2 == 0 || i % 7 == 0 {
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

    fn get_index(&self, row: u32, col: u32) -> usize {
        (row * self.width + col) as usize
    }

    fn live_neighbour_count(&self, row: u32, col: u32) -> u8 {
        let mut count = 0;
        for drow in -1..=1 {
            for dcol in -1..=1 {
                if (drow == 0) && (dcol == 0) {
                    continue;
                }
                let r = (row as i32 + drow) % self.height as i32;
                let c = (col as i32 + dcol) % self.width as i32;
                let ix = self.get_index(r as u32, c as u32);
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
                    (Cell::Alive, x) if x <= 2 => Cell::Dead,
                    (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
                    (Cell::Alive, x) if x > 3 => Cell::Dead,
                    (Cell::Dead, 3) => Cell::Alive,
                    _ => this_cell,
                };

                next[ix] = next_cell;
            }
        }
    }

    // for now, render to a string
    pub fn render(&self) -> String {
        self.to_string()
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
