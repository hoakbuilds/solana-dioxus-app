use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    pub type JsPublicKey;

    #[wasm_bindgen(method, js_name = toString)]
    pub fn to_string(this: &JsPublicKey) -> String;

    #[wasm_bindgen(method, js_name = toBuffer)]
    pub fn to_bytes(this: &JsPublicKey) -> Vec<u8>;
}
