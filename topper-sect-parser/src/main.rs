use futures::FutureExt;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::{
    window, CssStyleSheet, DomException, HtmlElement, HtmlIFrameElement, HtmlInputElement,
    SupportedType,
};
use yew::prelude::*;
mod loading;

enum SectMessage {
    Noop,
    Load(JsFuture),
    Loaded(String),
    Error(String),
}

impl From<()> for SectMessage {
    fn from(_: ()) -> Self {
        SectMessage::Noop
    }
}

struct SectModel {
    loading: bool,
    loaded: Option<String>,
    error: Option<String>,
}

async fn load_sect_log(result: Result<JsValue, JsValue>) -> SectMessage {
    match result {
        Ok(loaded) => SectMessage::Loaded(loaded.as_string().unwrap_or("Not a string".to_string())),
        Err(error) => SectMessage::Error(
            error
                .as_string()
                .unwrap_or("Could not parse error!".to_string()),
        ),
    }
}

impl Component for SectModel {
    type Message = SectMessage;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            loading: false,
            loaded: None,
            error: None,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            SectMessage::Load(future) => {
                ctx.link().send_future(future.then(load_sect_log));
                self.loading = true;
                true
            }
            SectMessage::Loaded(loaded) => {
                self.loaded = Some(loaded);
                true
            }
            SectMessage::Error(error) => {
                self.loading = false;
                self.error = Some(error);
                true
            }
            SectMessage::Noop => false,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        let on_sect_log = link.batch_callback(|e: Event| {
            e.target()
                .and_then(|target| target.dyn_into::<HtmlInputElement>().ok())
                .and_then(|target| target.files())
                .and_then(|files| files.get(0))
                .map(|file| SectMessage::Load(file.text().into()))
        });
        if let Some(loaded) = &self.loaded {
            let onload = link.callback(move |e: Event| {
                e.target()
                    .and_then(|target| target.dyn_into::<HtmlIFrameElement>().ok())
                    .map(|frame| {
                        let outer_style_sheet: JsValue = window()
                            .unwrap()
                            .document()
                            .unwrap()
                            .style_sheets()
                            .get(0)
                            .unwrap()
                            .into();
                        let inner_style_sheet: JsValue = frame
                            .content_document()
                            .unwrap()
                            .style_sheets()
                            .get(0)
                            .unwrap()
                            .into();
                        let outer_style_sheet: CssStyleSheet = outer_style_sheet.into();
                        let rules = outer_style_sheet.css_rules().unwrap();
                        let inner_style_sheet: CssStyleSheet = inner_style_sheet.into();
                        for idx in 0..rules.length() {
                            let rule = rules.get(idx).unwrap();
                            inner_style_sheet.insert_rule(rule.css_text().as_ref());
                        }
                    });
            });
            return html! {
                <iframe width="100%" height="100%" srcdoc={loaded.to_string()} {onload}></iframe>
            };
        }
        html! {
            <div>
                if let Some(error) = &self.error {
                    <p>{error}</p>
                }
                <label for="sect_log">{"Select a Sect log file:"}</label>
                <input type="file" id="sect_log" name="sect_log" onchange={on_sect_log}/>
            </div>
        }
    }
}

fn main() {
    yew::start_app::<SectModel>();
}
