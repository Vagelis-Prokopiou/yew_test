// Colyseus

use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = Colyseus)]
    // pub type Client;

    // #[wasm_bindgen(constructor)]
    // pub fn new(url: String) -> Client;

    // #[wasm_bindgen(method)]
    pub fn Client(url: String) -> JsValue;




    // This is async
    // #[wasm_bindgen(method)]
    // pub fn joinOrCreate(this: &Client, roomName: String);
}