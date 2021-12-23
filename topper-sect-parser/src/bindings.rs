use wasm_bindgen::prelude::*;
use web_sys::HtmlIFrameElement;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
    // Adds a message listener to the window, to receive from the iframe!
    pub fn add_message_listener(f: &Closure<dyn Fn(usize)>);
    // Adds a scroll listener to the iframe.
    pub fn add_scroll_listener(frame: &HtmlIFrameElement, f: &Closure<dyn Fn(i32)>);
}
