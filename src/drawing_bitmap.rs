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

// reverse byte order: A, B, G, R
// alpha needs to be 0xFF so it's not transparent
fn parse_color(color_str: &str) -> Result<u32, String> {
    let s = color_str.trim_start_matches("#");
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
