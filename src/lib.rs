mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    // imported FROM JS so we can interact with it
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    // exported TO JS so this function can be called
    alert("Hello, wasm-game-of-life!");
}
