use core::slice;
use std::iter;

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
    cell_size: u32,
}

#[wasm_bindgen]
impl RenderPixels {
    pub fn new_from(
        universe: &Universe,
        cell_size: u32,
        live_color: &str,
        dead_color: &str,
    ) -> Result<RenderPixels, String> {
        let live_color = parse_color(live_color)?;
        let dead_color = parse_color(dead_color)?;

        let width = universe.width() as usize * cell_size as usize;
        let height = universe.height() as usize * cell_size as usize;
        let length = width * height;
        let pixel_buffer = vec![0; length].into_boxed_slice();
        Ok(RenderPixels {
            pixel_buffer,
            width,
            height,
            live_color,
            dead_color,
            cell_size,
        })
    }

    pub fn wasm_draw_pixels(
        &mut self,
        ctx: &CanvasRenderingContext2d,
        universe: &Universe,
    ) -> Result<(), JsValue> {
        let cells = universe.cells.as_slice();
        let cell_size = self.cell_size as usize;

        iter::zip(
            cells.chunks_exact(universe.width as usize),
            self.pixel_buffer
                .chunks_exact_mut(self.width * self.cell_size as usize),
        )
        .for_each(|(universe_row, scanlines_n)| {
            // paint first scanline
            universe_row
                .iter()
                .zip(scanlines_n.chunks_exact_mut(cell_size))
                .for_each(|(cell, cell_pixels)| {
                    cell_pixels.fill(match cell {
                        Cell::Alive => self.live_color,
                        Cell::Dead => self.dead_color,
                    });
                });

            // repeat first scanline over remaining scan lines for this row of cells
            let mut it = scanlines_n.chunks_exact_mut(self.width);
            let first = it.next().unwrap();
            for line in it {
                line.copy_from_slice(first);
            }
        });

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

// reverse byte order: A, B, G, R
// alpha needs to be 0xFF so it's not transparent
fn parse_color(color_str: &str) -> Result<u32, String> {
    let s = color_str.trim_start_matches('#');
    let n = u32::from_str_radix(s, 16)
        .map_err(|e| format!("Could not parse color {s} due to error {e}"))?;

    match s.len() {
        6 => Ok((n << 8 | 0xff).to_be()), // assume alpha is 0xff if note supplied
        8 => Ok(n.to_be()),
        _ => Err(format!("Could not parse color {s} due to incorrect length")),
    }
}

#[cfg(test)]
mod tests {
    use super::parse_color;

    #[test]
    fn parse_valid_6_color() {
        let col = "#010abc";
        let res = parse_color(col).unwrap();
        println!("{col} -> {res:x}");
        assert_eq!(res, 0xff_bc_0a_01);
    }

    #[test]
    fn parse_valid_8_color() {
        let col = "#010abc7f";
        let res = parse_color(col).unwrap();
        assert_eq!(res, 0x7f_bc_0a_01);
    }

    #[test]
    fn parse_invalid_colors() {
        parse_color("#010abcgg").expect_err("expecting invalid number");
        parse_color("#abcde").expect_err("expecting wrong length");
    }
}
