use core::slice;

use wasm_bindgen::{prelude::*, Clamped};
use web_sys::{CanvasRenderingContext2d, ImageData};

use crate::{Cell, Universe};

#[wasm_bindgen]
pub struct RenderPixels {
    pixel_buffer: Box<[u32]>,
    width: usize,
    height: usize,
    live_color: u32,
    dead_color: u32,
    grid_color: u32,
    cell_size: u32,
}

#[wasm_bindgen]
impl RenderPixels {
    pub fn new_from(universe: &Universe, cell_size: u32) -> Self {
        // reverse byte order: A, B, G, R
        // alpha needs to be 0xFF so it's not transparent

        // const GRID_COLOR = "#004000";
        // const DEAD_COLOR = "#001000";
        // const LIVE_COLOR = "#00FF00";

        let live_color: u32 = 0xFF00FF00;
        let dead_color: u32 = 0xFF001000;
        let grid_color: u32 = 0xFF004000;

        let width = universe.width() as usize * cell_size as usize;
        let height = universe.height() as usize * cell_size as usize;
        let length = width * height;
        let pixel_buffer = vec![0; length].into_boxed_slice();
        RenderPixels {
            pixel_buffer,
            width,
            height,
            live_color,
            dead_color,
            grid_color,
            cell_size,
        }
    }

    pub fn wasm_draw_pixels(
        &mut self,
        ctx: &CanvasRenderingContext2d,
        universe: &Universe,
    ) -> Result<(), JsValue> {
        self.pixel_buffer.fill(self.dead_color);

        // draw live cells
        let cell_size = self.cell_size as usize;
        for (_, (row, col)) in universe
            .cells()
            .iter()
            .zip(universe.addresses_iter())
            .filter(|(&cell, _)| cell == Cell::Alive)
        {
            // draw rectangle (scan lines)
            let start_row = row as usize * cell_size;
            for r in 0..cell_size {
                let start = (start_row + r) * self.width + col as usize * cell_size;
                let end = start + cell_size;
                self.pixel_buffer[start..end].fill(self.live_color);
            }
        }

        // reinterpret [u32] as [u8]
        let bytes = unsafe {
            slice::from_raw_parts(
                self.pixel_buffer.as_ptr() as *const u8,
                self.pixel_buffer.len() * 4,
            )
        };

        // create image
        let img = ImageData::new_with_u8_clamped_array_and_sh(
            Clamped(bytes),
            self.width as u32,
            self.height as u32,
        )?;

        ctx.put_image_data(&img, 0.0, 0.0)?;

        Ok(())
    }
}
