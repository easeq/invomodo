use wasm_bindgen::prelude::*;

// pub mod domains;
pub mod app;
pub mod auth;
pub mod components;
pub mod routes;
pub mod shared;

// Import the `console.log` function from JavaScript
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}
