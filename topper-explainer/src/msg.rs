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
    ExplainerPageMessage(ExplainerPageMessage),
    Error(String),
}

impl From<ExplainerPageMessage> for ExplainerMessage {
    fn from(value: ExplainerPageMessage) -> Self {
        Self::ExplainerPageMessage(value)
    }
}

impl From<Option<ExplainerMessage>> for ExplainerMessage {
    fn from(value: Option<ExplainerMessage>) -> Self {
        value.unwrap_or(ExplainerMessage::Noop)
    }
}
