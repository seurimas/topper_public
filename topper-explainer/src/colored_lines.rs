use regex::Regex;
use topper_core::colored_lines::{get_colored_text, get_content_of_colored_text};
use yew::{prelude::*, virtual_dom::VNode};

pub fn render_line_with_color(line: &String) -> (Html, String) {
    let mut rendered = Html::default();
    let colored_text = get_colored_text(line);
    if let VNode::VList(list) = &mut rendered {
        for (color, text) in colored_text.iter() {
            list.push(html!(<span
                style={format!("color: {}", color)}>
                { text }
            </span>));
        }
    }
    (rendered, get_content_of_colored_text(&colored_text))
}
