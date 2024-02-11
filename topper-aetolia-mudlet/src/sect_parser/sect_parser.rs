use regex::Regex;
use std::{cmp::Ordering, marker::PhantomData};
use tl::*;
use topper_aetolia::{
    explainer::ExplainerPage,
    timeline::{AetPrompt, AetTimeSlice, AetTimeline},
};
use topper_core::colored_lines::get_content_of_raw_colored_text;

lazy_static! {
    pub static ref PROMPT_REGEX: Regex =
        Regex::new(r"\[(?P<hour>\d\d):(?P<minute>\d\d):(?P<second>\d\d):(?P<centi>\d\d)\]")
            .unwrap();
    static ref WHO_REGEX: Regex = Regex::new(r"^Who:\s+(?P<who>\w+)$").unwrap();
    static ref VS_REGEX: Regex = Regex::new(r"^Vs:\s+(?P<vs>\w+)$").unwrap();
    static ref CLASS_REGEX: Regex = Regex::new(r"^Class:\s+(?P<class>\w+)$").unwrap();
}

#[derive(Debug)]
pub struct AetoliaSectParser {
    last_color: String,
    lines: Vec<String>,
    line_remaining: String,
    time: String,
    me: String,
    you: String,
    my_class: String,
    your_class: String,
}

fn get_pre_block(body: &VDom) -> Option<NodeHandle> {
    body.query_selector("pre").and_then(|mut pre| pre.next())
}

impl AetoliaSectParser {
    pub fn new() -> Self {
        Self {
            last_color: String::new(),
            lines: Vec::new(),
            line_remaining: String::new(),
            time: String::new(),
            me: String::new(),
            you: String::new(),
            my_class: String::new(),
            your_class: String::new(),
        }
    }

    pub fn parse_nodes(&mut self, log_text: String) -> Result<ExplainerPage, String> {
        let document =
            tl::parse(&log_text, ParserOptions::default()).map_err(|err| format!("{:?}", err))?;
        let pre_block = get_pre_block(&document).ok_or_else(|| format!("No pre block found"))?;
        let children = pre_block
            .get(document.parser())
            .unwrap()
            .children()
            .ok_or_else(|| format!("No children"))?;
        for child in children.top().iter() {
            let node = child.get(document.parser()).unwrap();
            let color = get_color_from_node(&node);
            let text = node
                .inner_text(document.parser())
                .replace("&gt;", ">")
                .replace("&lt;", "<")
                .replace("&amp;", "&")
                .replace("&quot;", "\"")
                .replace("&apos;", "'");
            if self.time.is_empty() {
                if let Some(captures) = WHO_REGEX.captures(&text) {
                    if let Some(who) = captures.name("who") {
                        self.me = who.as_str().to_string();
                    }
                } else if let Some(captures) = VS_REGEX.captures(&text) {
                    if let Some(vs) = captures.name("vs") {
                        self.you = vs.as_str().to_string();
                    }
                } else if let Some(captures) = CLASS_REGEX.captures(&text) {
                    if let Some(class) = captures.name("class") {
                        if self.you.is_empty() {
                            self.my_class = class.as_str().to_string();
                        } else {
                            self.your_class = class.as_str().to_string();
                        }
                    }
                } else if let Some(captures) = PROMPT_REGEX.captures(&text) {
                    self.time = format!(
                        "{}_{}_{}",
                        captures.name("hour").unwrap().as_str(),
                        captures.name("minute").unwrap().as_str(),
                        captures.name("second").unwrap().as_str()
                    );
                }
            }
            self.append_colored_text(text.to_string(), color);
        }

        let id = format!(
            "{}_{}_vs_{}_{}_{}_{}",
            self.me,
            self.my_class,
            self.you,
            self.your_class,
            self.time,
            self.lines.len()
        );
        Ok(ExplainerPage::new(id, self.lines.clone()))
    }

    fn append_colored_text(&mut self, mut text: String, color: String) {
        if !self.last_color.eq(&color) && text.trim().len() > 0 {
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

pub fn parse_me_and_you(page: &ExplainerPage) -> (String, String) {
    let mut me = "Unknown".to_string();
    let mut you = "Assailant".to_string();
    for line_text in page.get_body().iter() {
        let text = get_content_of_raw_colored_text(line_text);
        if let Some(captures) = WHO_REGEX.captures(&text) {
            if let Some(who) = captures.name("who") {
                me = who.as_str().to_string();
            }
        } else if let Some(captures) = VS_REGEX.captures(&text) {
            if let Some(vs) = captures.name("vs") {
                you = vs.as_str().to_string();
            }
        } else if let Some(matches) = PROMPT_REGEX.find(&text) {
            break;
        }
    }
    (me, you)
}

pub fn parse_prompt_time(line: &String, last_time: i32) -> Option<i32> {
    if let Some(captures) = PROMPT_REGEX.captures(line.as_ref()) {
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
            return Some(time);
        }
    }
    None
}

pub fn is_prompt(line: &String) -> bool {
    parse_prompt_time(line, 0).is_some()
}

pub fn get_color_from_node(node: &Node) -> String {
    let mut color = "white".to_string();
    if let Some(tag) = node.as_tag() {
        if let Some(Some(style)) = tag.attributes().get("style") {
            let style = style.as_utf8_str();
            if let Some(color_index) = style.find("color:") {
                let color_start = color_index + 6;
                let color_end = style[color_start..].find(';').unwrap();
                color = style[color_start..color_start + color_end].to_string();
            }
        }
    }
    // TODO: Get the color from the node
    color
}
