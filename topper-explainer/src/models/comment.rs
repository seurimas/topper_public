use crate::{bindings::log, explainer::Comment};
use serde::{Deserialize, Serialize};
use wasm_bindgen::{JsCast, UnwrapThrowExt};
use web_sys::HtmlDivElement;
use yew::prelude::*;

pub struct CommentBlock {
    editing: bool,
    new_val: String,
}

#[derive(Properties, PartialEq)]
pub struct CommentBlockProperties {
    pub comment: Comment,
    pub edit_mode: bool,
    pub on_change: Callback<String>,
    pub on_delete: Callback<()>,
    pub on_close: Callback<()>,
}

pub enum CommentMessage {
    Change(String),
    Edit,
    Finish,
    Close,
}

fn get_value_from_input_event(e: InputEvent) -> String {
    let event: Event = e.dyn_into().unwrap_throw();
    let event_target = event.target().unwrap_throw();
    let target: HtmlDivElement = event_target.dyn_into().unwrap_throw();
    web_sys::console::log_1(&target.text_content().into());
    target.inner_text()
}

impl Component for CommentBlock {
    type Properties = CommentBlockProperties;
    type Message = CommentMessage;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            editing: ctx.props().comment.is_empty(),
            new_val: String::new(),
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();
        html!(<div class="page__comment">
          {if self.editing {
            let oninput = ctx.link().callback(|event: InputEvent| {
                CommentMessage::Change(get_value_from_input_event(event))
            });
            crate::bindings::log(&format!("Body: {}", self.new_val));
            html!(<div class="page__comment__editor" contenteditable="true" oninput={oninput}>{self.new_val.clone()}</div>)
          } else {
            html!(<div class="page__comment__text">{props.comment.body.clone()}</div>)
          }}
          {if props.edit_mode {
            if self.editing {
                let onclick = ctx.link().callback(|_| CommentMessage::Finish);
                Some(html!(<button class="page__comment__finish" onclick={onclick}>{ "Finish" }</button>))
            } else {
                let onclick = ctx.link().callback(|_| CommentMessage::Edit);
                Some(html!(<button class="page__comment__edit" onclick={onclick}>{ "Edit" }</button>))
            }
          } else {
            None
          }}
          {if !self.editing {
            let onclick = ctx.link().callback(|_| CommentMessage::Close);
            Some(html!(<button class="page__comment__close" onclick={onclick}>{ "Close" }</button>))
          } else {
            None
          }}
        </div>)
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            CommentMessage::Edit => {
                self.editing = true;
                self.new_val = ctx.props().comment.body.clone();
                true
            }
            CommentMessage::Finish => {
                log(&format!("{:?}", self.new_val));
                if self.new_val.trim().is_empty() {
                    ctx.props().on_delete.emit(());
                } else {
                    ctx.props().on_change.emit(self.new_val.clone());
                }
                self.editing = false;
                self.new_val = String::new();
                true
            }
            CommentMessage::Change(new_val) => {
                self.new_val = new_val;
                true
            }
            CommentMessage::Close => {
                ctx.props().on_close.emit(());
                true
            }
        }
    }
}
