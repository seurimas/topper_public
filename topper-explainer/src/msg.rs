use wasm_bindgen_futures::JsFuture;

use crate::explainer::page::ExplainerPageMessage;
use crate::explainer::ExplainerPage;

pub enum ExplainerMessage {
    Noop,
    Load(JsFuture),
    Loaded(ExplainerPage),
    ExplainerPageMessage(ExplainerPageMessage),
    Error(String),
}

impl From<ExplainerPageMessage> for ExplainerMessage {
    fn from(value: ExplainerPageMessage) -> Self {
        Self::ExplainerPageMessage(value)
    }
}
