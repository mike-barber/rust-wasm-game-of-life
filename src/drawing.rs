use crate::{Cell, Universe};
use wasm_bindgen::prelude::*;
use web_sys::CanvasRenderingContext2d;

const GRID_COLOR: &str = "#CCCCCC";
const DEAD_COLOR: &str = "#FFFFFF";
const LIVE_COLOR: &str = "#000000";



#[wasm_bindgen]
pub fn wasm_draw_cells(ctx: &CanvasRenderingContext2d, cell_size: f64, universe: &Universe) {
    ctx.begin_path();

    let live_color = JsValue::from_str(LIVE_COLOR);
    let dead_color = JsValue::from_str(DEAD_COLOR);

    let cells = universe.get_cells();

    // TODO: this would probably be faster and more efficient with a zip
    // over rows/cols, cells; calculating index and dereferencing is
    // inefficient.

    // draw live cells
    ctx.set_fill_style(&live_color);
    for row in 0..universe.height {
        for col in 0..universe.width {
            let idx = universe.get_index(row, col);
            if cells[idx] == Cell::Alive {
                ctx.fill_rect(
                    col as f64 * cell_size,
                    row as f64 * cell_size,
                    cell_size,
                    cell_size,
                );
            }
        }
    }

    // draw dead cells
    ctx.set_fill_style(&dead_color);
    for row in 0..universe.height {
        for col in 0..universe.width {
            let idx = universe.get_index(row, col);
            if cells[idx] == Cell::Dead {
                ctx.fill_rect(
                    col as f64 * cell_size,
                    row as f64 * cell_size,
                    cell_size,
                    cell_size,
                );
            }
        }
    }

    ctx.stroke();
}

#[wasm_bindgen]
pub fn wasm_draw_grid(ctx: &CanvasRenderingContext2d, cell_size: f64, universe: &Universe) {
    let grid_color = JsValue::from_str(GRID_COLOR);
    let height = universe.height() as f64;
    let width = universe.width() as f64;

    ctx.begin_path();
    ctx.set_stroke_style(&grid_color);

    // vertical lines
    let y_end = cell_size * height;
    for i in 0..=universe.width() {
        let xi = i as f64 * cell_size;
        ctx.move_to(xi, 0.0);
        ctx.line_to(xi, y_end);
    }

    // horizontal lines
    let x_end = cell_size * width;
    for i in 0..=universe.height() {
        let yi = i as f64 * cell_size;
        ctx.move_to(0.0, yi);
        ctx.line_to(x_end, yi);
    }

    ctx.stroke();
}
