use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn greet(name: &str) -> String {
    format!("Hello, {} from Rust!", name)
}

#[wasm_bindgen]
pub fn create_window(title: &str) {
    // Logic to create the Window
    println!("Creating window: {}", title);
}
