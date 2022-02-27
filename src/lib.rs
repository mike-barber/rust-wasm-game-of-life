pub mod drawing;
mod utils;

use std::fmt::Display;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// A macro to provide `println!(..)`-style syntax for `console.log` logging.
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

impl Cell {
    pub fn flip(&mut self) {
        *self = match *self {
            Cell::Dead => Cell::Alive,
            Cell::Alive => Cell::Dead,
        }
    }

    pub fn random() -> Cell {
        if js_sys::Math::random() < 0.5 {
            Cell::Alive
        } else {
            Cell::Dead
        }
    }
}

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
    flip: Option<Vec<Cell>>,
}

#[wasm_bindgen]
impl Universe {
    pub fn new() -> Universe {
        // Configure panic hook (not sure this is the right place, since
        // it's quite a global thing. It'll do for now.
        utils::set_panic_hook();

        // create the universe
        let width = 256;
        let height = 256;

        log!("Creating Universe with dimensions {} x {}", width, height);

        let cells = (0..width * height).map(|_i| Cell::random()).collect();
        let flip = Some(vec![Cell::Dead; (width * height) as usize]);

        Universe {
            width,
            height,
            cells,
            flip,
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
        self.flip = Some(vec![Cell::Dead; (self.width * self.height) as usize]);
    }

    pub fn set_height(&mut self, height: u32) {
        self.height = height;
        self.cells = vec![Cell::Dead; (self.width * self.height) as usize];
        self.flip = Some(vec![Cell::Dead; (self.width * self.height) as usize]);
    }

    pub fn reset_random(&mut self) {
        self.cells.iter_mut().for_each(|v| *v = Cell::random());
    }

    pub fn reset_zero(&mut self) {
        self.cells.iter_mut().for_each(|v| *v = Cell::Dead);
    }

    pub fn cells_ptr(&self) -> *const Cell {
        self.cells.as_ptr()
    }

    fn get_index(&self, row: u32, col: u32) -> usize {
        (row * self.width + col) as usize
    }

    pub fn flip_cell(&mut self, row: u32, col: u32) {
        let idx = self.get_index(row, col);
        self.cells[idx].flip();
    }

    fn live_neighbour_count_edge(&self, row: u32, col: u32) -> u8 {
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

    fn live_neighbour_count_centre(&self, row_context: [&[Cell]; 3], col: u32) -> u8 {
        let col = col as usize;

        let [r0, r1, r2] = row_context;

        let top = r0[col - 1] as u8 + r0[col] as u8 + r0[col + 1] as u8;
        let mid = r1[col - 1] as u8 + r1[col + 1] as u8;
        let bot = r2[col - 1] as u8 + r2[col] as u8 + r2[col + 1] as u8;

        top + mid + bot
    }

    fn live_neighbour_count_row_context(&self, row: u32) -> [&[Cell]; 3] {
        let width = self.width as usize;
        let row = row as usize;

        let s1 = row * width;
        let s0 = s1 - width;
        let s2 = s1 + width;
        [
            &self.cells[s0..(s0 + width)],
            &self.cells[s1..(s1 + width)],
            &self.cells[s2..(s2 + width)],
        ]
    }

    fn next_cell(this_cell: Cell, live_neighbours: u8) -> Cell {
        match (this_cell, live_neighbours) {
            (Cell::Alive, x) if x < 2 => Cell::Dead,
            (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
            (Cell::Alive, x) if x > 3 => Cell::Dead,
            (Cell::Dead, 3) => Cell::Alive,
            _ => this_cell,
        }
    }

    // This approach is more complicated, but reduces the time for each
    // tick from about 16ms to 2-3ms. It could be optimised further,
    // of course.
    pub fn tick(&mut self) {
        // take flip buffer out of self to decouple it for mutability
        let mut flip = self.flip.take().unwrap();

        // top and bottom edge
        let r0 = 0;
        let rh = self.height - 1;
        for col in 0..self.width {
            let ix0 = self.get_index(r0, col);
            let ixh = self.get_index(rh, col);

            let l0 = self.live_neighbour_count_edge(r0, col);
            let lh = self.live_neighbour_count_edge(rh, col);

            flip[ix0] = Self::next_cell(self.cells[ix0], l0);
            flip[ixh] = Self::next_cell(self.cells[ixh], lh);
        }

        // left and right edge
        let c0 = 0;
        let cw = self.width - 1;
        for row in 1..(self.height - 1) {
            let ix0 = self.get_index(row, c0);
            let ixw = self.get_index(row, cw);

            let l0 = self.live_neighbour_count_edge(row, c0);
            let lw = self.live_neighbour_count_edge(row, cw);

            flip[ix0] = Self::next_cell(self.cells[ix0], l0);
            flip[ixw] = Self::next_cell(self.cells[ixw], lw);
        }

        // centre section (more efficiently)
        for row in 1..(self.height - 1) {
            let row_context = self.live_neighbour_count_row_context(row);
            for col in 1..(self.width - 1) {
                let ix = self.get_index(row, col);
                let lc = self.live_neighbour_count_centre(row_context, col);

                flip[ix] = Self::next_cell(self.cells[ix], lc);
            }
        }

        // swap buffers and return flip to the structure
        std::mem::swap(&mut self.cells, &mut flip);
        self.flip = Some(flip);
    }

    // for now, render to a string
    pub fn render(&self) -> String {
        self.to_string()
    }
}

// Note: NOT part of the exposed interface to bindgen; for internal use
impl Universe {
    pub fn cells(&self) -> &[Cell] {
        &self.cells
    }

    pub fn set_cells(&mut self, cell_addresses: &[(u32, u32)]) {
        for (r, c) in cell_addresses.iter().cloned() {
            let ix = self.get_index(r, c);
            self.cells[ix] = Cell::Alive;
        }
    }

    // iterator over (row,column) addresses
    pub fn addresses_iter(&self) -> impl Iterator<Item = (u32, u32)> + '_ {
        (0..self.height).flat_map(move |r| (0..self.width).map(move |c| (r, c)))
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

pub struct ScopeTimer<'a> {
    name: &'a str,
}

impl<'a> ScopeTimer<'a> {
    pub fn new(name: &'a str) -> ScopeTimer<'a> {
        web_sys::console::time_with_label(name);
        ScopeTimer { name }
    }
}

impl<'a> Drop for ScopeTimer<'a> {
    fn drop(&mut self) {
        web_sys::console::time_end_with_label(self.name);
    }
}
