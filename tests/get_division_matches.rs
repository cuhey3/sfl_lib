//! Test suite for the Web and headless browsers.
#![cfg(target_arch = "wasm32")]

use sfl_lib;
use sfl_lib::get_division_matches;
extern crate wasm_bindgen_test;
use sfl_lib::greet;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_node_experimental);

#[wasm_bindgen_test]
fn pass() {
    get_division_matches();
    assert_eq!(1 + 1, 2);
}
