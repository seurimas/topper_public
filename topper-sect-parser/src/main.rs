#[macro_use]
extern crate lazy_static;
use bindings::*;
use futures::FutureExt;
use timeline::{parse_time_slices, update_timeline};
use topper_aetolia::timeline::{AetTimeSlice, AetTimeline};
use wasm_bindgen::closure::Closure;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::{
    window, CssStyleSheet, DomException, Element, HtmlElement, HtmlIFrameElement, HtmlInputElement,
    HtmlPreElement, Node,
};
use yew::prelude::*;
mod bindings;
mod loading;
mod timeline;

enum SectMessage {
    Noop,
    Load(JsFuture),
    Loaded(String),
    Error(String),
    InnerMessage(usize),
    Scroll(usize),
    Initialize(Vec<Element>),
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
    line_nodes: Vec<Element>,
    time_slices: Vec<AetTimeSlice>,
    timeline: AetTimeline,
}

fn push_styles_into_frame(frame: &HtmlIFrameElement) {
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
}

fn rearrange_lines(frame: &HtmlIFrameElement) -> Vec<Element> {
    let document = frame.content_document().unwrap();
    let body = document.body().unwrap();
    let pre_block: HtmlPreElement = body.child_nodes().get(1).unwrap().dyn_into().unwrap();
    let mut lines: Vec<Vec<Node>> = vec![];
    let mut line: Vec<Node> = vec![];
    for node_idx in 0..pre_block.child_element_count() {
        let node = pre_block.child_nodes().get(node_idx).unwrap();
        if let Some(text) = node.text_content() {
            line.push(node);
            if text.ends_with("\n") {
                let new_line = line;
                line = vec![];
                lines.push(new_line);
            }
        }
    }
    let mut new_lines = vec![];
    for line in lines.iter() {
        let new_line = document.create_element("span").unwrap();
        new_line.set_attribute("class", "line").unwrap();
        for node in line.iter() {
            new_line.append_child(node).unwrap();
        }
        body.append_child(&new_line).unwrap();
        new_lines.push(new_line);
    }
    new_lines
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

    fn create(ctx: &Context<Self>) -> Self {
        log("Adding message listener...");
        let link = ctx.link().clone();
        let handler: Box<dyn Fn(usize)> = Box::new(move |value| {
            link.send_message(SectMessage::InnerMessage(value));
        });
        let closure = Closure::wrap(handler);
        add_message_listener(&closure);
        closure.forget();
        Self {
            loading: false,
            loaded: None,
            error: None,
            line_nodes: vec![],
            time_slices: vec![],
            timeline: AetTimeline::new(),
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
            SectMessage::InnerMessage(line_idx) => {
                log(format!("Received {}", line_idx).as_ref());
                update_timeline(&mut self.timeline, &self.time_slices, line_idx);
                log(format!("{:?}", self.timeline.state.borrow_me()).as_ref());
                false
            }
            SectMessage::Scroll(scroll_top) => {
                log(format!("Scrolled {}", scroll_top).as_ref());
                false
            }
            SectMessage::Initialize(line_nodes) => {
                self.time_slices = parse_time_slices(&line_nodes);
                log(format!("{:?}", self.time_slices).as_ref());
                self.line_nodes = line_nodes;
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
            let cb_link = link.clone();
            let onload = link.callback(move |e: Event| {
                let iframe = e
                    .target()
                    .and_then(|target| target.dyn_into::<HtmlIFrameElement>().ok());
                if let Some(iframe) = iframe {
                    let cb_link = cb_link.clone();
                    let handler: Box<dyn Fn(usize)> = Box::new(move |value| {
                        cb_link.send_message(SectMessage::Scroll(value));
                    });
                    let closure = Closure::wrap(handler);
                    add_scroll_listener(&iframe, &closure);
                    closure.forget();
                    push_styles_into_frame(&iframe);
                    let log_lines = rearrange_lines(&iframe);
                    for (log_idx, log_line) in log_lines.iter().enumerate() {
                        let log_line: JsValue = log_line.into();
                        let log_line: HtmlElement = log_line.into();
                        log_line.set_onclick(Some(&js_sys::Function::new_no_args(
                            format!("window.parent.postMessage({}, \"*\");", log_idx).as_ref(),
                        )));
                    }
                    SectMessage::Initialize(log_lines)
                } else {
                    SectMessage::Noop
                }
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
