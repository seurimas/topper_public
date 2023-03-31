use js_sys::Promise;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
    // Fetches a log from a CORS proxy.
    pub fn fetch_file(s: &str) -> Promise;
}
