//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]
extern crate wasm_bindgen_test;
use std::assert_eq;

#[cfg(test)]
use wasm_ar::universe::Universe;

use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);
