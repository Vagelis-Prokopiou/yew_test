// Colyseus

use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
extern "C" {
    pub type Client;

    #[wasm_bindgen(constructor, js_namespace = Colyseus)]
    pub fn new(url: String) -> Client;

    // Todo: This is async, but it works (for demo purposes).
    #[wasm_bindgen(method)]
    pub fn joinOrCreate(this: &Client, roomName: String);
}