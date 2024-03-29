use wasm_bindgen_futures::JsFuture;
use web_sys::HtmlIFrameElement;

use crate::explainer::ExplainerPage;

pub enum ExplainerMessage {
    Noop,
    LoadPage(JsFuture),
    LoadedPage(ExplainerPage),
    LoadedPublished(Vec<String>),
    LoadFile(JsFuture),
    LoadedFile(String),
    SetTime(i32),
    InitializeSect(HtmlIFrameElement),
    Error(String),
}
