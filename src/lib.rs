mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn something_owned() -> String{
    String::from("oh hai")
}

#[wasm_bindgen]
pub fn take_js_value_by_value(x: JsValue) -> JsValue {
    x
}
