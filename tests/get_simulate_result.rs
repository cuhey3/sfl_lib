//! Test suite for the Web and headless browsers.
#![cfg(target_arch = "wasm32")]

use sfl_lib;
extern crate wasm_bindgen_test;
use sfl_lib::greet;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_node_experimental);

#[wasm_bindgen_test]
fn pass() {
    greet(js_sys::Array::new(), true);
    assert_eq!(1 + 1, 2);
}
