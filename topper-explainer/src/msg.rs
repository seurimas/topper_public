use wasm_bindgen_futures::JsFuture;
use web_sys::HtmlIFrameElement;

use crate::explainer::page::ExplainerPageMessage;
use crate::explainer::ExplainerPage;

pub enum ExplainerMessage {
    Noop,
    LoadPage(JsFuture),
    LoadedPage(ExplainerPage),
    LoadFile(JsFuture),
    LoadedFile(String),
    InitializeSect(HtmlIFrameElement),
    Error(String),
}
