#[macro_use]
extern crate lazy_static;
use crate::battle_stats::{get_battle_stats, BattleStatsElem};
use crate::manipulations::{push_styles_into_frame, rearrange_lines};
use bindings::*;
use futures::FutureExt;
use manipulations::{find_scroll_point, get_scroll_points, move_scroll_indicator};
use timeline::{parse_time_slices, update_timeline};
use topper_aetolia::timeline::{AetTimeSlice, AetTimeline};
use wasm_bindgen::closure::Closure;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::{window, Element, HtmlElement, HtmlIFrameElement, HtmlInputElement};
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
    Initialize(HtmlIFrameElement, Vec<Element>),
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
    iframe_element: Option<HtmlIFrameElement>,
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
        Err(error) => SectMessage::Error(format!("Could not parse: {:?}", error)),
    }
}

impl SectModel {
    fn update_scroll_point(&mut self, line_idx: usize) {
        let scroll_top = *self.scroll_points.get(line_idx).unwrap_or(&0);
        self.last_scroll_point = scroll_top;
        if let Some(iframe_element) = &self.iframe_element {
            move_scroll_indicator(iframe_element, scroll_top);
        }
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
        let check_sect_log_link = ctx
            .link()
            .clone()
            .batch_callback(|_| {
                let window = window().unwrap();
                let location = window.location();
                if let Ok(mut sect_log_link) = location.search() {
                    if sect_log_link.len() == 0 {
                        return None;
                    }
                    sect_log_link.remove(0);
                    Some(SectMessage::Load(fetch_log(sect_log_link.as_ref()).into()))
                } else {
                    None
                }
            })
            .emit(());
        Self {
            loading: false,
            loaded: None,
            error: None,
            iframe_element: None,
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
                if let Some(new_line_idx) =
                    update_timeline(&mut self.timeline, &self.time_slices, line_idx)
                {
                    self.update_scroll_point(new_line_idx);
                }
                log(format!("{:?}", self.timeline.state.borrow_me()).as_ref());
                true
            }
            SectMessage::Scroll(scroll_top) => {
                let scroll_point = scroll_top + 300;
                if scroll_point > self.last_scroll_point {
                    let line_idx = find_scroll_point(&self.scroll_points, scroll_point);
                    if let Some(new_line_idx) =
                        update_timeline(&mut self.timeline, &self.time_slices, line_idx)
                    {
                        self.update_scroll_point(new_line_idx);
                    }
                    true
                } else {
                    false
                }
            }
            SectMessage::Initialize(iframe_element, line_nodes) => {
                let (me, target, time_slices) = parse_time_slices(&line_nodes);
                self.me = me;
                self.target = target;
                self.time_slices = time_slices;
                self.scroll_points = get_scroll_points(&line_nodes);
                log(format!("{} {} {:?}", self.me, self.target, self.time_slices).as_ref());
                self.iframe_element = Some(iframe_element);
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
        let on_sect_log_link = link.batch_callback(|e: Event| {
            e.target()
                .and_then(|target| target.dyn_into::<HtmlInputElement>().ok())
                .map(|target| target.value())
                .map(|link| SectMessage::Load(fetch_log(link.as_ref()).into()))
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
                    SectMessage::Initialize(iframe, log_lines)
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
                <label for="sect_log_link">{"Or enter a link:"}</label>
                <input type="text" id="sect_log_link" name="sect_log_link" onchange={on_sect_log_link}/>
            </div>
        }
    }
}

fn main() {
    yew::start_app::<SectModel>();
}
