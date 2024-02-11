use serde::{Deserialize, Serialize};
use topper_aetolia::timeline::{AetTimeSlice, AetTimelineState};
use yew::{prelude::*, virtual_dom::VNode};

use crate::{
    bindings::{export_json, get_time, is_unlocked, log},
    explainer::ExplainerPage,
    models::{comment::CommentBlock, line::PageLine, state::StateBlock},
    sect_parser::{build_time_slices, get_timeline_state, parse_me_and_you},
};

use crate::explainer::{Comment, Mutation};

#[derive(Default, Debug)]
pub struct ExplainerPageModel {
    page: ExplainerPage,
    me: String,
    you: String,
    time_slices: Vec<AetTimeSlice>,
    viewing_state: Option<(usize, AetTimelineState)>,
    viewing_comments: Vec<usize>,
    edit_mode: bool,
    pass_msg: Callback<ExplainerPageMessage>,
}

#[derive(Debug)]
pub enum ExplainerPageMessage {
    BeginNewComment(usize),
    OpenComment(usize),
    CloseComment(usize),
    UpdateComment(usize, String),
    DeleteComment(usize),
    ToggleState(usize),
    ToggleEditMode,
    ToggleExpanded,
    Export,
}

impl ExplainerPageModel {
    fn is_expanded(&self) -> bool {
        self.viewing_comments.len() == self.page.comments.len()
    }

    fn render_line(
        &self,
        ctx: &Context<Self>,
        idx: usize,
        line: &String,
        on_msg: Callback<ExplainerPageMessage>,
    ) -> VNode {
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
        let state_block = if let Some((viewing_idx, viewing_state)) = self.viewing_state.as_ref() {
            if *viewing_idx != idx {
                None
            } else {
                Some(html!(<StateBlock
                    state={viewing_state.clone()}
                    me={self.me.clone()}
                    you={self.you.clone()}
                />))
            }
        } else {
            None
        };
        html!(<PageLine
            key={idx}
            idx={idx}
            has_comment={has_comment}
            comment_open={comment_block.is_some()}
            line={line.clone()}
            edit_mode={self.edit_mode}
            msg={on_msg}
        >
          {comment_block}
          {state_block}
        </PageLine>)
    }
}

#[derive(Properties, PartialEq)]
pub struct ExplainerPageProperties {
    pub page: ExplainerPage,
}

impl Component for ExplainerPageModel {
    type Message = ExplainerPageMessage;
    type Properties = ExplainerPageProperties;
    fn create(ctx: &Context<Self>) -> Self {
        let page = &ctx.props().page;
        let locked = page.locked && !is_unlocked();
        let (me, you) = parse_me_and_you(&page);
        Self {
            edit_mode: !locked,
            viewing_comments: if locked {
                page.get_comment_lines()
            } else {
                Vec::new()
            },
            viewing_state: None,
            page: page.clone(),
            me,
            you,
            time_slices: build_time_slices(&page),
            pass_msg: ctx.link().callback(|msg| msg),
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let page_lines = self
            .page
            .body
            .iter()
            .enumerate()
            .map(|(idx, line)| self.render_line(ctx, idx, line, self.pass_msg.clone()));
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
                <div key="page" class="page">
                    {edit_section}
                    {expand_button}
                    <div class="page__lines">{ for page_lines }</div>
                </div>
            </>
        )
    }

    fn update(&mut self, ctx: &Context<Self>, msg: ExplainerPageMessage) -> bool {
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
            ExplainerPageMessage::ToggleState(line_idx) => {
                let (me, _you) = parse_me_and_you(&self.page);
                let timeline_state = get_timeline_state(me, &self.time_slices, line_idx as u32);
                log(&format!("{:?}", timeline_state.borrow_me()));
                self.viewing_state = Some((line_idx, timeline_state));
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
                let mut page = self.page.clone();
                if !self.edit_mode {
                    page.locked = true;
                } else {
                    page.locked = false;
                }
                match serde_json::to_string(&page) {
                    Ok(exported) => export_json(&exported),
                    Err(err) => log(&format!("{:?}", err)),
                }
                true
            }
        }
    }
}
