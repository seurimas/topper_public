use serde::{Deserialize, Serialize};
use yew::{prelude::*, virtual_dom::VNode};

use crate::{
    bindings::{export_json, get_time, is_unlocked, log},
    explainer::{comment::CommentBlock, line::PageLine},
};

use super::{Comment, ExplainerModel};

#[derive(Default, Debug)]
pub struct ExplainerPageModel {
    page: ExplainerPage,
    viewing_comments: Vec<usize>,
    edit_mode: bool,
}

#[derive(Debug)]
pub enum ExplainerPageMessage {
    BeginNewComment(usize),
    OpenComment(usize),
    CloseComment(usize),
    UpdateComment(usize, String),
    DeleteComment(usize),
    ToggleEditMode,
    ToggleExpanded,
    Export,
}

#[derive(Default, Debug, Deserialize, Serialize, PartialEq)]
pub struct ExplainerPage {
    pub id: String,
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

    pub fn get_comment_lines(&self) -> Vec<usize> {
        self.comments
            .iter()
            .map(|comment| comment.get_line())
            .collect()
    }

    pub fn update_comment(&mut self, line: usize, new_val: String) {
        self.comments
            .iter_mut()
            .filter(|comment| comment.is_for_line(line))
            .next()
            .map(move |comment| comment.update_body(new_val));
    }

    pub fn delete_comment(&mut self, line: usize) {
        self.comments.retain(|comment| !comment.is_for_line(line));
    }
}

impl ExplainerPageModel {
    pub fn new(page: ExplainerPage) -> Self {
        let locked = page.locked && !is_unlocked();
        Self {
            edit_mode: !locked,
            viewing_comments: if locked {
                page.get_comment_lines()
            } else {
                Vec::new()
            },
            page,
        }
    }

    fn is_expanded(&self) -> bool {
        self.viewing_comments.len() == self.page.comments.len()
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
                let on_close = ctx
                    .link()
                    .callback(move |_| ExplainerPageMessage::CloseComment(idx));
                html!(<CommentBlock
                    comment={comment.clone()}
                    edit_mode={self.edit_mode}
                    on_change={on_change}
                    on_delete={on_delete}
                    on_close={on_close}
                />)
            })
        } else {
            None
        };
        html!(<PageLine
            idx={idx}
            has_comment={has_comment}
            comment_open={comment_block.is_some()}
            line={line.clone()}
            edit_mode={self.edit_mode}
            msg={ctx.link().callback(|msg| msg)}
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
        let edit_section = if self.page.locked && !is_unlocked() {
            None
        } else {
            let export_button = {
                let export = ctx.link().callback(|_| ExplainerPageMessage::Export);
                html!(<div class="page__save_button" onclick={export}>{ "Export to JSON" }</div>)
            };
            let edit_button = {
                let toggle_edit = ctx
                    .link()
                    .callback(|_| ExplainerPageMessage::ToggleEditMode);
                html!(<div class="page__edit_toggle" onclick={toggle_edit}>{ if !self.edit_mode { "Edit" } else { "Finish" } }</div>)
            };
            Some(html!(<div class="page__edit_section">{edit_button}{export_button}</div>))
        };
        let expand_button = {
            let expanded = self.is_expanded();
            let toggle_expand = ctx
                .link()
                .callback(|_| ExplainerPageMessage::ToggleExpanded);
            html!(<div class="page__expand_button" onclick={toggle_expand}>{if expanded { "Collapse Comments" } else { "Open All Comments" }}</div>)
        };
        html!(
            <>
                <a id="export"></a>
                <div class="page">
                    {edit_section}
                    {expand_button}
                    <div class="page__lines">{ for page_lines }</div>
                </div>
            </>
        )
    }

    pub fn update(&mut self, ctx: &Context<ExplainerModel>, msg: ExplainerPageMessage) -> bool {
        match msg {
            ExplainerPageMessage::BeginNewComment(line) => {
                self.page.comments.push(Comment::new(line, get_time()));
                self.viewing_comments.push(line);
                true
            }
            ExplainerPageMessage::OpenComment(line_idx) => {
                if !self.viewing_comments.contains(&line_idx) {
                    self.viewing_comments.push(line_idx);
                }
                true
            }
            ExplainerPageMessage::CloseComment(line_idx) => {
                self.viewing_comments.retain(|idx| *idx != line_idx);
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
            ExplainerPageMessage::ToggleExpanded => {
                if self.is_expanded() {
                    self.viewing_comments = Vec::new();
                } else {
                    self.viewing_comments = self.page.get_comment_lines();
                }
                true
            }
            ExplainerPageMessage::Export => {
                match serde_json::to_string(&self.page) {
                    Ok(exported) => export_json(&exported),
                    Err(err) => log(&format!("{:?}", err)),
                }
                true
            }
        }
    }
}
