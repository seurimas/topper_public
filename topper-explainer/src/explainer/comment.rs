use serde::{Deserialize, Serialize};
use wasm_bindgen::{JsCast, UnwrapThrowExt};
use web_sys::HtmlTextAreaElement;
use yew::prelude::*;

use super::{page::ExplainerPageMessage, ExplainerPage};

#[derive(PartialEq, Clone, Deserialize, Serialize)]
pub struct Comment {
    body: String,
    reference_line: usize,
}

impl Comment {
    pub fn new(reference_line: usize) -> Self {
        Self {
            body: String::new(),
            reference_line,
        }
    }

    pub fn is_for_line(&self, line: usize) -> bool {
        self.reference_line == line
    }

    pub fn is_empty(&self) -> bool {
        self.body.is_empty()
    }

    pub fn update_body(&mut self, body: String) {
        self.body = body;
    }
}

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
}

pub enum CommentMessage {
    Change(String),
    Edit,
    Finish,
}

fn get_value_from_input_event(e: InputEvent) -> String {
    let event: Event = e.dyn_into().unwrap_throw();
    let event_target = event.target().unwrap_throw();
    let target: HtmlTextAreaElement = event_target.dyn_into().unwrap_throw();
    web_sys::console::log_1(&target.value().into());
    target.value()
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
            html!(<textarea class="page__comment__editor" oninput={oninput}>{props.comment.body.clone()}</textarea>)
          } else {
            html!(<div class="page__comment__text">{props.comment.body.clone()}</div>)
          }}
          {if props.edit_mode {
            Some(html!(<button class="page__comment__edit">{ "Edit" }</button>))
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
                if self.new_val.is_empty() {
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
        }
    }
}
