use js_sys::Promise;
use wasm_bindgen::prelude::*;
use web_sys::Node;
use yew::Callback;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);

    pub fn trace(s: &str);

    pub fn toggle_playback(speed: f32, time: i32);
    pub fn remember_playback_cb(f: &Closure<dyn Fn(i32)>);
    pub fn update_playback_time(time: i32);
    pub fn update_playback_speed(time: f32);

    // Log a node for debugging purposes.
    pub fn debug_node(val: &JsValue);

    // Gets the computed color of a node
    pub fn get_color_from_node(node: &Node) -> String;

    // Exports to a JSON file for sharing
    pub fn export_json(s: &str);

    // Returns true if the page is unlocked for editing, despite the JSON value.
    pub fn is_unlocked() -> bool;

    // Autoscrolls the page to the bottom, once.
    pub fn autoscroll_once();

    // Sets the document title text.
    pub fn set_title(s: &str);

    // Fetches the current time in milliseconds.
    pub fn get_time() -> i32;

    // Fetches a log based on the URL provided.
    pub fn fetch_file(s: &str) -> Promise;

    pub fn ttsSpeak(s: &str);
    pub fn ttsQueue(s: &str);
}
