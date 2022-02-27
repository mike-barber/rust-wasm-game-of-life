use crate::{Cell, Universe};
use wasm_bindgen::prelude::*;
use web_sys::CanvasRenderingContext2d;

#[wasm_bindgen]
pub struct RenderSettings {
    cell_size: f64,
    live_color: JsValue,
    dead_color: JsValue,
    grid_color: JsValue,
}

#[wasm_bindgen]
impl RenderSettings {
    pub fn new(
        cell_size: f64,
        live_color: JsValue,
        dead_color: JsValue,
        grid_color: JsValue,
    ) -> RenderSettings {
        RenderSettings {
            cell_size,
            live_color,
            dead_color,
            grid_color,
        }
    }
}

#[wasm_bindgen]
pub fn wasm_draw_cells(
    ctx: &CanvasRenderingContext2d,
    settings: &RenderSettings,
    universe: &Universe,
) {
    ctx.begin_path();

    let cell_size = settings.cell_size;
    let live_color = &settings.live_color;
    let dead_color = &settings.dead_color;

    // draw live cells
    ctx.set_fill_style(live_color);
    for (_, (row, col)) in universe
        .cells()
        .iter()
        .zip(universe.addresses_iter())
        .filter(|(&cell, _)| cell == Cell::Alive)
    {
        ctx.fill_rect(
            col as f64 * cell_size,
            row as f64 * cell_size,
            cell_size,
            cell_size,
        );
    }

    // draw dead cells
    ctx.set_fill_style(dead_color);
    for (_, (row, col)) in universe
        .cells()
        .iter()
        .zip(universe.addresses_iter())
        .filter(|(&cell, _)| cell == Cell::Dead)
    {
        ctx.fill_rect(
            col as f64 * cell_size,
            row as f64 * cell_size,
            cell_size,
            cell_size,
        );
    }

    ctx.stroke();
}

#[wasm_bindgen]
pub fn wasm_draw_grid(
    ctx: &CanvasRenderingContext2d,
    settings: &RenderSettings,
    universe: &Universe,
) {
    let grid_color = &settings.grid_color;
    let cell_size = settings.cell_size;
    let height = universe.height() as f64;
    let width = universe.width() as f64;

    ctx.begin_path();
    ctx.set_stroke_style(grid_color);

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
