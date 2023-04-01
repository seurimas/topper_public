use regex::Regex;
use yew::{prelude::*, virtual_dom::VNode};

use crate::bindings::log;

use super::page::ExplainerPageMessage;

pub struct PageLine;

#[derive(Properties, PartialEq)]
pub struct PageLineProperties {
    pub children: Children,
    pub idx: usize,
    pub has_comment: bool,
    pub comment_open: bool,
    pub line: AttrValue,
    pub edit_mode: bool,
    pub msg: Callback<ExplainerPageMessage>,
}

lazy_static! {
    static ref COLOR: Regex = Regex::new(r"<(?P<color>[^>]+)>").unwrap();
}

fn render_line_with_color(line: String) -> (Html, String) {
    let mut rendered = VNode::default();
    let mut seen = 0;
    let mut active_color = None;
    let mut content = String::new();
    if let VNode::VList(list) = &mut rendered {
        for capture in COLOR.captures_iter(&line) {
            let color = capture.get(1).unwrap();
            let color_start = color.start() - 1;
            if color_start > seen {
                let text = &line[seen..color_start];
                list.push(html!(<span
                    style={format!("color: {}", active_color.unwrap_or("white"))}>
                    { text }
                </span>));
                content.push_str(text);
            }
            seen = color.end() + 1;
            active_color = Some(color.as_str());
        }
        if seen < line.len() {
            let text = &line[seen..];
            list.push(html!(<span
                style={format!("color: {}", active_color.unwrap_or("white"))}>
                {text}
            </span>));
            content.push_str(text);
        }
    }
    (rendered, content)
}

impl Component for PageLine {
    type Message = ExplainerPageMessage;
    type Properties = PageLineProperties;

    fn create(ctx: &Context<Self>) -> Self {
        Self
    }
    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();
        let (rendered_line, line_content) = render_line_with_color(props.line.to_string());
        let comment_icon = if !props.comment_open && props.has_comment {
            let idx = props.idx;
            let open_comment = ctx
                .link()
                .callback(move |_| ExplainerPageMessage::OpenComment(idx));
            Some(html!(<div class="page__open_comment" onclick={open_comment}>{"\""}</div>))
        } else if !props.comment_open && props.edit_mode && line_content.trim().len() > 0 {
            let idx = props.idx;
            let add_comment = ctx
                .link()
                .callback(move |_| ExplainerPageMessage::BeginNewComment(idx));
            Some(html!(<div class="page__add_comment" onclick={add_comment}>{"+"}</div>))
        } else {
            None
        };
        html!(<>
            <div class="page__line">
                <span class="page__line__text">{ rendered_line }</span>
                {comment_icon}
            </div>
            {for props.children.iter()}
        </>)
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        log(&format!("{:?}", msg));
        ctx.props().msg.emit(msg);
        false
    }
}
