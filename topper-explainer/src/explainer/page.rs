use serde::{Deserialize, Serialize};
use yew::{prelude::*, virtual_dom::VNode};

use crate::explainer::{comment::CommentBlock, line::PageLine};

use super::{Comment, ExplainerModel};

#[derive(Default)]
pub struct ExplainerPageModel {
    page: ExplainerPage,
    viewing_comments: Vec<usize>,
    edit_mode: bool,
    expanded_mode: bool,
}

#[derive(Default, Deserialize, Serialize, PartialEq)]
pub struct ExplainerPage {
    body: Vec<String>,
    #[serde(default)]
    comments: Vec<Comment>,
    #[serde(default)]
    locked: bool,
}

impl ExplainerPage {
    pub fn get_comment(&self, line: usize) -> Option<Comment> {
        self.comments
            .iter()
            .filter(|comment| comment.is_for_line(line))
            .cloned()
            .next()
    }

    pub fn update_comment(&mut self, line: usize, new_val: String) {
        self.comments[line].update_body(new_val);
    }

    pub fn delete_comment(&mut self, line: usize) {
        self.comments.remove(line);
    }
}

impl ExplainerPageModel {
    pub fn new(page: ExplainerPage) -> Self {
        Self {
            edit_mode: !page.locked,
            page,
            viewing_comments: Vec::new(),
            expanded_mode: false,
        }
    }

    fn render_line(&self, ctx: &Context<ExplainerModel>, idx: usize, line: &String) -> VNode {
        let comment = self.page.get_comment(idx);
        let has_comment = comment.is_some();
        let comment_block = if self.viewing_comments.contains(&idx) {
            comment.map(|comment| {
                let on_change = ctx
                    .link()
                    .callback(move |val| ExplainerPageMessage::UpdateComment(idx, val));
                let on_delete = ctx
                    .link()
                    .callback(move |_| ExplainerPageMessage::DeleteComment(idx));
                html!(<CommentBlock
                    comment={comment.clone()}
                    edit_mode={self.edit_mode}
                    on_change={on_change}
                    on_delete={on_delete}
                />)
            })
        } else {
            None
        };
        html!(<PageLine
            idx={idx}
            has_comment={has_comment}
            line={line.clone()}
            edit_mode={self.edit_mode}
        >
          {comment_block}
        </PageLine>)
    }

    pub fn view(&self, ctx: &Context<ExplainerModel>) -> Html {
        let page_lines = self
            .page
            .body
            .iter()
            .enumerate()
            .map(|(idx, line)| self.render_line(ctx, idx, line));
        let edit_button = if self.page.locked {
            None
        } else {
            let toggle_edit = ctx
                .link()
                .callback(|_| ExplainerPageMessage::ToggleEditMode);
            Some(html!(<div class="page__edit_toggle" onclick={toggle_edit}>{ "Edit" }</div>))
        };
        let expand_button = {
            let toggle_expand = ctx
                .link()
                .callback(|_| ExplainerPageMessage::ToggleExpandedMode);
            html!(<div class="page__expanded_toggle" onclick={toggle_expand}>{ "Expanded View" }</div>)
        };
        html!(
            <div class="page">
              {edit_button}
              {expand_button}
              <div class="page__lines">{ for page_lines }</div>
            </div>
        )
    }

    pub fn update(&mut self, ctx: &Context<ExplainerModel>, msg: ExplainerPageMessage) -> bool {
        match msg {
            ExplainerPageMessage::BeginNewComment(line) => {
                self.page.comments.push(Comment::new(line));
                self.viewing_comments.push(line);
                true
            }
            ExplainerPageMessage::ToggleComment(line_idx) => {
                if self.viewing_comments.contains(&line_idx) {
                    self.viewing_comments.retain(|idx| *idx != line_idx);
                } else {
                    self.viewing_comments.push(line_idx);
                }
                true
            }
            ExplainerPageMessage::DeleteComment(line_idx) => {
                self.page.delete_comment(line_idx);
                true
            }
            ExplainerPageMessage::UpdateComment(line_idx, new_val) => {
                self.page.update_comment(line_idx, new_val);
                true
            }
            ExplainerPageMessage::ToggleEditMode => {
                self.edit_mode = !self.edit_mode;
                true
            }
            ExplainerPageMessage::ToggleExpandedMode => {
                self.expanded_mode = !self.expanded_mode;
                true
            }
        }
    }
}

pub enum ExplainerPageMessage {
    BeginNewComment(usize),
    ToggleComment(usize),
    UpdateComment(usize, String),
    DeleteComment(usize),
    ToggleEditMode,
    ToggleExpandedMode,
}
