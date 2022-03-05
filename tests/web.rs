//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[cfg(test)]
mod tests {

    use wasm_bindgen_test::*;
    use wasm_game_of_life::Universe;

    #[wasm_bindgen_test]
    fn iterate_succeeds() {
        // arrange
        let mut universe = Universe::new(256,256).unwrap();

        // act & assert
        universe.tick();
    }

    pub fn input_spaceship() -> Universe {
        let mut universe = Universe::new(6,6).unwrap();
        universe.set_cells(&[(1, 2), (2, 3), (3, 1), (3, 2), (3, 3)]);
        universe
    }

    pub fn expected_spaceship() -> Universe {
        let mut universe = Universe::new(6,6).unwrap();
        universe.set_cells(&[(2, 1), (2, 3), (3, 2), (3, 3), (4, 2)]);
        universe
    }

    #[wasm_bindgen_test]
    pub fn test_tick() {
        // arrange
        let mut input_universe = input_spaceship();
        let expected_universe = expected_spaceship();

        // act
        input_universe.tick();

        // assert
        assert_eq!(&input_universe.cells(), &expected_universe.cells());
    }

    #[wasm_bindgen_test]
    fn addresses_iterator_test() {
        // arrange
        let universe = Universe::new(256,256).unwrap();
        
        // act + assert        
        let mut iter = universe.addresses_iter();
        for r in 0..universe.height() {
            for c in 0..universe.width() {
                let (ri, ci) = iter.next().unwrap();
                assert_eq!(r, ri);
                assert_eq!(c, ci);
            }
        }
    }
}
