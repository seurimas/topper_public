use wasm_bindgen::JsCast;
use web_sys::*;
use yew::prelude::*;

use crate::{models::ExplainerModel, msg::ExplainerMessage};

pub fn load_sect_into_iframe(ctx: &Context<ExplainerModel>, loaded: &String) -> Html {
    let link = ctx.link();
    let onload = link.callback(move |e: Event| {
        let iframe = e
            .target()
            .and_then(|target| target.dyn_into::<HtmlIFrameElement>().ok());
        if let Some(iframe) = iframe {
            ExplainerMessage::InitializeSect(iframe)
        } else {
            ExplainerMessage::Noop
        }
    });
    html!(<iframe height="100%" srcdoc={loaded.to_string()} {onload}></iframe>)
}
