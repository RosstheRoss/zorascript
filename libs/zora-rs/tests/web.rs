//! Test suite for the Web and headless browsers.

#![cfg(target_family = "wasm")]

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;
use zora_rs::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn pass() {
    assert_eq!(1 + 1, 2);
}
