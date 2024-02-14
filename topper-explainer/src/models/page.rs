use serde::{Deserialize, Serialize};
use topper_aetolia::timeline::{AetObservation, AetTimeSlice, AetTimeline, AetTimelineState};
use topper_core::timeline::{db::DummyDatabaseModule, BaseTimeline};
use yew::{prelude::*, virtual_dom::VNode};

use crate::{
    bindings::{export_json, get_time, is_unlocked, log, trace},
    explainer::ExplainerPage,
    models::{comment::CommentBlock, line::PageLine, state::StateBlock},
    sect_parser::{build_line_times, build_time_slices, parse_me_and_you},
};

use crate::explainer::{Comment, Mutation};

use super::{ttsQueue, ttsSpeak};

#[derive(Default, Debug)]
pub struct ExplainerPageModel {
    page: ExplainerPage,
    me: String,
    you: String,
    time_slices: Vec<AetTimeSlice>,
    line_times: Vec<(usize, i32)>,
    viewing_state: Option<(usize, AetTimeline)>,
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
        last_line: bool,
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
            if *viewing_idx != idx || last_line {
                None
            } else {
                Some(html!(<StateBlock
                    timeline={viewing_state.clone()}
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

    fn get_last_line_for_time(&self, time: Option<i32>) -> Option<usize> {
        time.map(|time| {
            let mut last_line = 0;
            for (line_idx, line_time) in self.line_times.iter() {
                if *line_time > time {
                    break;
                }
                last_line = *line_idx;
            }
            last_line
        })
    }

    fn set_timeline_state(&mut self, me: String, prompt_line_idx: usize) -> Vec<AetTimeSlice> {
        if let Some((last_line_idx, last_timeline)) = self.viewing_state.as_mut() {
            if prompt_line_idx > *last_line_idx {
                let mut new_time_slices = Vec::new();
                for slice in &self.time_slices {
                    if slice
                        .lines
                        .iter()
                        .find(|(_line, idx)| *idx > prompt_line_idx as u32)
                        .is_some()
                    {
                        break;
                    }
                    if slice
                        .lines
                        .iter()
                        .find(|(_line, idx)| *idx > *last_line_idx as u32)
                        .is_some()
                    {
                        last_timeline
                            .push_time_slice(slice.clone(), None as Option<&DummyDatabaseModule>);
                        new_time_slices.push(slice.clone());
                    }
                }
                *last_line_idx = prompt_line_idx as usize;
                return new_time_slices;
            }
        }
        let mut timeline = AetTimeline::new();
        timeline.state.me = me;
        for slice in &self.time_slices {
            if slice
                .lines
                .iter()
                .find(|(_line, idx)| *idx > prompt_line_idx as u32)
                .is_some()
            {
                break;
            }
            timeline.push_time_slice(slice.clone(), None as Option<&DummyDatabaseModule>);
        }
        self.viewing_state = Some((prompt_line_idx as usize, timeline.clone()));
        vec![]
    }

    fn view_state(&mut self, line_idx: usize) -> Vec<AetTimeSlice> {
        let (me, _you) = parse_me_and_you(&self.page);
        self.set_timeline_state(me, line_idx)
    }

    fn callout_combat_actions(&self, new_slices: Vec<AetTimeSlice>) {
        if new_slices.len() > 10 {
            return;
        }
        for slice in new_slices {
            for observation in slice.observations.iter().flatten() {
                match observation {
                    AetObservation::CombatAction(action) => {
                        if action.caster == self.you {
                            ttsQueue(&format!("{}", action.skill));
                        }
                        trace(&format!("{:?}", action));
                    }
                    _ => {}
                }
            }
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct ExplainerPageProperties {
    pub page: ExplainerPage,
    pub time: Option<i32>,
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
            line_times: build_line_times(&page),
            pass_msg: ctx.link().callback(|msg| msg),
        }
    }

    fn changed(&mut self, ctx: &Context<Self>, old_props: &Self::Properties) -> bool {
        let last_time = old_props.time;
        let new_time = ctx.props().time;
        if let (Some(last_time), Some(new_time)) = (last_time, new_time) {
            if new_time != last_time {
                let last_last_line = self.get_last_line_for_time(Some(last_time));
                let new_last_line = self.get_last_line_for_time(Some(new_time));
                if let (Some(last_last_line), Some(new_last_line)) = (last_last_line, new_last_line)
                {
                    if last_last_line != new_last_line {
                        let new_slices = self.view_state(new_last_line);
                        self.callout_combat_actions(new_slices);
                        return true;
                    }
                }
            }
        }
        if ctx.props().page != old_props.page {
            true
        } else if old_props.time.is_none() && ctx.props().time.is_some() {
            true
        } else {
            false
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let last_time_line = self.get_last_line_for_time(ctx.props().time);
        let page_lines = self
            .page
            .body
            .iter()
            .enumerate()
            .filter(|(line_idx, _)| {
                if let Some(last_line) = last_time_line {
                    *line_idx <= last_line
                } else {
                    true
                }
            })
            .map(|(idx, line)| {
                self.render_line(
                    ctx,
                    idx,
                    line,
                    self.pass_msg.clone(),
                    Some(idx) == last_time_line,
                )
            });
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
        let end_state = if let Some((prompt_line, timeline)) = self.viewing_state.as_ref() {
            if Some(*prompt_line) == last_time_line {
                Some(html!(<StateBlock
                    timeline={timeline.clone()}
                    me={self.me.clone()}
                    you={self.you.clone()}
                />))
            } else {
                None
            }
        } else {
            None
        };
        html!(
            <>
                <a id="export"></a>
                <div key="page" class="page">
                    {edit_section}
                    {expand_button}
                    <div class="page__lines">{ for page_lines }</div>
                    {end_state}
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
                self.view_state(line_idx);
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
