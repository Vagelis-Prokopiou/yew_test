use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen(module = "/js/lib_a.js")]
extern "C" {
    // Standalone function definition
    pub fn getIsoDate() -> String;

    // Class definition
    pub type MyClass;

    #[wasm_bindgen(constructor)]
    pub fn new(number: u32) -> MyClass;

    #[wasm_bindgen(method, getter)]
    pub fn number(this: &MyClass) -> u32;
    #[wasm_bindgen(method, setter)]
    pub fn set_number(this: &MyClass, number: u32) -> MyClass;
    #[wasm_bindgen(method)]
    pub fn render(this: &MyClass) -> String;
}