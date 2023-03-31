use yew::prelude::*;

use super::{page::ExplainerPageMessage, Comment};

pub struct PageLine;

#[derive(Properties, PartialEq)]
pub struct PageLineProperties {
    pub children: Children,
    pub idx: usize,
    pub has_comment: bool,
    pub line: AttrValue,
    pub edit_mode: bool,
}

impl Component for PageLine {
    type Message = ExplainerPageMessage;
    type Properties = PageLineProperties;

    fn create(ctx: &Context<Self>) -> Self {
        Self
    }
    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();
        let comment_icon = if ctx.props().edit_mode {
            if props.has_comment {
                let idx = props.idx;
                let open_comment = ctx
                    .link()
                    .callback(move |_| ExplainerPageMessage::ToggleComment(idx));
                Some(html!(<div class="page__open_comment" onclick={open_comment}>{"*"}</div>))
            } else {
                let idx = props.idx;
                let add_comment = ctx
                    .link()
                    .callback(move |_| ExplainerPageMessage::BeginNewComment(idx));
                Some(html!(<div class="page__add_comment" onclick={add_comment}>{"+"}</div>))
            }
        } else {
            None
        };
        html!(<div class="page__line"><span class="page__line__text">{ props.line.clone() }</span>{comment_icon}</div>)
    }
}
