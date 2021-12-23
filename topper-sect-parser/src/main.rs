#[macro_use]
extern crate lazy_static;
use crate::battle_stats::{get_battle_stats, BattleStatsElem};
use crate::manipulations::{push_styles_into_frame, rearrange_lines};
use bindings::*;
use futures::FutureExt;
use manipulations::{find_scroll_point, get_scroll_points};
use timeline::{parse_time_slices, update_timeline};
use topper_aetolia::timeline::{AetTimeSlice, AetTimeline};
use wasm_bindgen::closure::Closure;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::{Element, HtmlElement, HtmlIFrameElement, HtmlInputElement};
use yew::prelude::*;
mod battle_stats;
mod bindings;
mod manipulations;
mod timeline;

enum SectMessage {
    Noop,
    Load(JsFuture),
    Loaded(String),
    Error(String),
    InnerMessage(usize),
    Scroll(i32),
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
    last_scroll_point: i32,
    scroll_points: Vec<i32>,
    time_slices: Vec<AetTimeSlice>,
    timeline: AetTimeline,
    me: String,
    target: String,
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
            last_scroll_point: 0,
            scroll_points: vec![],
            time_slices: vec![],
            timeline: AetTimeline::new(),
            me: String::new(),
            target: String::new(),
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
                self.last_scroll_point = *self.scroll_points.get(line_idx).unwrap_or(&0);
                log(format!("{:?}", self.timeline.state.borrow_me()).as_ref());
                true
            }
            SectMessage::Scroll(scroll_top) => {
                if scroll_top > self.last_scroll_point {
                    let line_idx = find_scroll_point(&self.scroll_points, scroll_top);
                    log(format!("Scrolled {} to {}", scroll_top, line_idx).as_ref());
                    self.last_scroll_point = scroll_top;
                    update_timeline(&mut self.timeline, &self.time_slices, line_idx)
                } else {
                    false
                }
            }
            SectMessage::Initialize(line_nodes) => {
                let (me, target, time_slices) = parse_time_slices(&line_nodes);
                self.me = me;
                self.target = target;
                self.time_slices = time_slices;
                self.scroll_points = get_scroll_points(&line_nodes);
                log(format!("{} {} {:?}", self.me, self.target, self.time_slices).as_ref());
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
                    let handler: Box<dyn Fn(i32)> = Box::new(move |value| {
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
            log("Rerendered...");
            return html! {
                <>
                    <iframe height="100%" srcdoc={loaded.to_string()} {onload}></iframe>
                    <BattleStatsElem me={true} battle_stats={get_battle_stats(&self.timeline, &self.me)} />
                    <BattleStatsElem me={false} battle_stats={get_battle_stats(&self.timeline, &self.target)} />
                </>
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
