use crate::{bindings::*, explainer::ExplainerPage};
use regex::Regex;
use std::{cmp::Ordering, marker::PhantomData};
use topper_aetolia::timeline::{AetPrompt, AetTimeSlice, AetTimeline};
use wasm_bindgen::JsCast;

use web_sys::*;

use super::observations::OBSERVER;

lazy_static! {
    static ref PROMPT_REGEX: Regex =
        Regex::new(r"\[(?P<hour>\d\d):(?P<minute>\d\d):(?P<second>\d\d):(?P<centi>\d\d)\]")
            .unwrap();
    static ref WHO_REGEX: Regex = Regex::new(r"^Who:\s+(?P<who>\w+)$").unwrap();
    static ref VS_REGEX: Regex = Regex::new(r"^Vs:\s+(?P<vs>\w+)$").unwrap();
}

#[derive(Debug)]
pub struct AetoliaSectParser {
    pub text: String,
    last_color: String,
    lines: Vec<String>,
    line_remaining: String,
    time: String,
    me: String,
    you: String,
}

fn get_pre_block(body: &HtmlElement) -> Option<HtmlPreElement> {
    let node_list = body.child_nodes();
    for idx in 0..node_list.length() {
        if node_list
            .get(idx)
            .unwrap()
            .node_name()
            .eq_ignore_ascii_case("pre")
        {
            return Some(node_list.get(idx).unwrap().unchecked_into());
        }
    }
    None
}

impl AetoliaSectParser {
    pub fn new(text: String) -> Self {
        Self {
            text,
            last_color: String::new(),
            lines: Vec::new(),
            line_remaining: String::new(),
            time: String::new(),
            me: String::new(),
            you: String::new(),
        }
    }

    pub fn parse_nodes(&mut self, frame: &HtmlIFrameElement) {
        let document = frame.content_document().unwrap();
        let body = document.body().unwrap();
        let pre_block: HtmlPreElement = get_pre_block(&body).unwrap();
        for node_idx in 0..pre_block.child_nodes().length() {
            let node = pre_block.child_nodes().get(node_idx).unwrap();
            let color = get_color_from_node(&node);
            // let color = "white".to_string();
            if let Some(text) = node.text_content() {
                if self.time.is_empty() {
                    if let Some(captures) = WHO_REGEX.captures(&text) {
                        if let Some(who) = captures.name("who") {
                            self.me = who.as_str().to_string();
                        }
                    } else if let Some(captures) = VS_REGEX.captures(&text) {
                        if let Some(vs) = captures.name("vs") {
                            self.you = vs.as_str().to_string();
                        }
                    } else if let Some(matches) = PROMPT_REGEX.find(&text) {
                        self.time = matches.as_str().to_string();
                    }
                }
                self.append_colored_text(text, color);
            } else {
                log(&format!("{:?}", node));
            }
        }

        // for line_node in line_nodes.iter() {
        //     let text = line_node.text_content().unwrap();
        //     debug_node(line_node);
        // }
    }

    pub fn get_page(&self) -> ExplainerPage {
        let id = format!("{} vs {} ({})", self.me, self.you, self.time);
        ExplainerPage::new(id, self.lines.clone())
    }

    fn append_colored_text(&mut self, mut text: String, color: String) {
        if !self.last_color.eq(&color) {
            self.line_remaining = format!("{}<{}>", self.line_remaining, color);
            self.last_color = color.clone();
        }
        while let Some((end_old, start_new)) = text.split_once("\n") {
            self.lines
                .push(format!("{}{}", self.line_remaining, end_old));
            self.line_remaining = format!("<{}>", color);
            text = start_new.to_string();
        }
        self.line_remaining = format!("{}{}", self.line_remaining, text);
    }
}

pub fn parse_time_slices(line_nodes: &Vec<Element>) -> (String, String, Vec<AetTimeSlice>) {
    let mut slices = vec![];
    let mut lines = vec![];
    let mut last_time = 0;
    let mut me = String::new();
    let mut you = String::new();
    for (line_idx, line_node) in line_nodes.iter().enumerate() {
        let line_text = line_node.text_content().unwrap();
        let line_text = line_text.trim().to_string();
        if line_text.contains("\n") {
            for line_text in line_text.split("\n") {
                lines.push((line_text.to_string(), line_idx as u32));
            }
        } else {
            lines.push((line_text.to_string(), line_idx as u32));
        }
        if let Some(captures) = WHO_REGEX.captures(line_text.as_ref()) {
            if let Some(who) = captures.name("who") {
                me = who.as_str().to_string();
            }
        } else if let Some(captures) = VS_REGEX.captures(line_text.as_ref()) {
            if let Some(vs) = captures.name("vs") {
                you = vs.as_str().to_string();
            }
        }
        if let Some(captures) = PROMPT_REGEX.captures(line_text.as_ref()) {
            if let (Some(hour), Some(minute), Some(second), Some(centi)) = (
                captures.name("hour"),
                captures.name("minute"),
                captures.name("second"),
                captures.name("centi"),
            ) {
                let hour: i32 = hour.as_str().parse().unwrap();
                let minute: i32 = minute.as_str().parse().unwrap();
                let second: i32 = second.as_str().parse().unwrap();
                let centi: i32 = centi.as_str().parse().unwrap();
                let mut time = centi + (((((hour * 60) + minute) * 60) + second) * 100);
                if time < last_time {
                    // It's a braaand neww day, and the sun is hiiigh.
                    time = time + (24 * 360000);
                }
                last_time = time;
                let mut slice = AetTimeSlice {
                    observations: None,
                    gmcp: Vec::new(),
                    lines: lines,
                    prompt: AetPrompt::Promptless,
                    time,
                    me: me.clone(),
                };
                slice.observations = Some(OBSERVER.observe(&slice));
                slices.push(slice);
                lines = vec![];
            }
        }
    }
    (me, you, slices)
}
