use regex::Regex;
use yew::{prelude::*, virtual_dom::VNode};

use crate::{bindings::log, colored_lines::render_line_with_color, sect_parser::is_prompt};

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

impl Component for PageLine {
    type Message = ExplainerPageMessage;
    type Properties = PageLineProperties;

    fn create(ctx: &Context<Self>) -> Self {
        Self
    }
    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();
        let (rendered_line, line_content) = render_line_with_color(&props.line.to_string());
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
        let state_icon = if is_prompt(&line_content) {
            let idx = props.idx;
            let view_state = ctx
                .link()
                .callback(move |_| ExplainerPageMessage::ToggleState(idx));
            Some(html!(<div class="page__view_state" onclick={view_state}>{"?"}</div>))
        } else {
            None
        };
        html!(<>
            <div class="page__line">
                <span class="page__line__text">{ rendered_line }</span>
                {state_icon}
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
