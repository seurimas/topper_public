use crate::{bindings::*, explainer::ExplainerPage, msg::ExplainerMessage};
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::JsFuture;
use web_sys::window;
use yew::prelude::*;

pub fn check_for_link<T: Component>(
    ctx: &Context<T>,
    msg: impl Fn(JsFuture) -> T::Message + 'static,
) {
    ctx.link()
        .clone()
        .batch_callback(move |_| {
            let window = window().unwrap();
            let location = window.location();
            if let Ok(mut link) = location.search() {
                if link.len() == 0 {
                    return None;
                }
                link.remove(0);
                Some(msg(fetch_file(link.as_ref()).into()))
            } else {
                None
            }
        })
        .emit(());
}

pub async fn load_page(result: Result<JsValue, JsValue>) -> ExplainerMessage {
    match result {
        Ok(loaded) => {
            let link_str = loaded.as_string().unwrap_or("Not a string".to_string());
            match serde_json::from_str(&link_str) {
                Ok(loaded) => ExplainerMessage::LoadedPage(loaded),
                Err(err) => ExplainerMessage::Error(format!("{:?}", err)),
            }
        }
        Err(error) => ExplainerMessage::Error(format!("Could not parse: {:?}", error)),
    }
}

pub async fn load_file(result: Result<JsValue, JsValue>) -> ExplainerMessage {
    match result {
        Ok(loaded) => {
            ExplainerMessage::LoadedFile(loaded.as_string().unwrap_or("Not a string".to_string()))
        }
        Err(error) => ExplainerMessage::Error(format!("Could not parse: {:?}", error)),
    }
}
