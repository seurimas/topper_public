use js_sys::Promise;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);

    // Exports to a JSON file for sharing
    pub fn export_json(s: &str);

    // Returns true if the page is unlocked for editing, despite the JSON value.
    pub fn is_unlocked() -> bool;

    // Sets the document title text.
    pub fn set_title(s: &str);

    // Fetches the current time in milliseconds.
    pub fn get_time() -> i32;

    // Fetches a log based on the URL provided.
    pub fn fetch_file(s: &str) -> Promise;
}
