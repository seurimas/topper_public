use crate::bindings::{
    autoscroll_once, remember_playback_cb, toggle_playback, update_playback_time,
};
use crate::sect_parser::parse_prompt_time;
use serde::{Deserialize, Serialize};
use topper_aetolia::explainer::ExplainerPage;
use topper_core::colored_lines::get_content_of_raw_colored_text;
use wasm_bindgen::closure::Closure;
use web_sys::HtmlElement;
use yew::prelude::*;

use super::update_playback_speed;

pub struct TimeControl {
    playback_speed_mod: f32,
}

pub enum TimeControlMessage {
    IncrementPlaybackSpeed,
    DecrementPlaybackSpeed,
}

#[derive(Properties, PartialEq)]
pub struct TimeControlProperties {
    pub start_time: i32,
    pub end_time: i32,
    pub time: Option<i32>,
    pub on_time_change: Callback<i32>,
}

impl Component for TimeControl {
    type Message = TimeControlMessage;
    type Properties = TimeControlProperties;

    fn create(ctx: &Context<Self>) -> Self {
        let link = ctx.link().clone();
        let on_time_change = ctx.props().on_time_change.clone();
        let handler: Box<dyn Fn(i32)> = Box::new(move |value| {
            on_time_change.emit(value);
        });
        let closure = Closure::wrap(handler);
        remember_playback_cb(&closure);
        closure.forget();
        TimeControl {
            playback_speed_mod: 0.75,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            TimeControlMessage::IncrementPlaybackSpeed => {
                self.playback_speed_mod += 0.05;
                update_playback_speed(self.playback_speed_mod);
                true
            }
            TimeControlMessage::DecrementPlaybackSpeed => {
                self.playback_speed_mod -= 0.05;
                update_playback_speed(self.playback_speed_mod);
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let duration = ctx.props().end_time - ctx.props().start_time;
        let progress = ctx
            .props()
            .time
            .map(|time| (time - ctx.props().start_time) as f32 / duration as f32)
            .unwrap_or_default()
            .min(1.);
        let start_time = ctx.props().start_time;
        let end_time = ctx.props().end_time;
        let time = ctx.props().time.unwrap_or(start_time);
        let on_time_change = ctx.props().on_time_change.clone();
        let onclick = Callback::from(move |evt: MouseEvent| {
            let new_progress = evt.offset_y() as f32
                / evt.target_unchecked_into::<HtmlElement>().client_height() as f32;
            let new_time = (new_progress * duration as f32) as i32 + start_time;
            on_time_change.emit(new_time);
            update_playback_time(new_time);
            autoscroll_once();
        });
        let playback_speed_mod = self.playback_speed_mod;
        let decrement_playback_speed = ctx
            .link()
            .callback(|_| TimeControlMessage::DecrementPlaybackSpeed);
        let increment_playback_speed = ctx
            .link()
            .callback(|_| TimeControlMessage::IncrementPlaybackSpeed);
        let on_time_change_end = ctx.props().on_time_change.clone();
        html! {
            <div class="time_control">
              <div class="time_control__label">
                {"Playback"}
              </div>
              <div class="time_control__playback">
                <div class="time_control__playback__button" onclick={Callback::from(move |_| {
                   toggle_playback(playback_speed_mod, time);
                })}>{"▶️"}</div>
                <div class="time_control__playback__decrement" onclick={decrement_playback_speed}>{"-"}</div>
                <div class="time_control__playback__speed">{format!("{:.0}%", self.playback_speed_mod * 100.0)}</div>
                <div class="time_control__playback__increment" onclick={increment_playback_speed}>{"+"}</div>
              </div>
                <div
                  class="time_control__display"
                  onclick={onclick}
                >
                  <div
                    class="time_control__display__filled" style={format!("height: {}%", progress * 100.0)}
                  />
                </div>
                <div class="time_control__end" onclick={Callback::from(move |_| {
                    on_time_change_end.emit(end_time);
                    update_playback_time(end_time);
                    autoscroll_once();
                })}>{"Go To End"}</div>
            </div>
        }
    }
}

pub fn get_start_and_end_time(page: &ExplainerPage) -> Option<(i32, i32)> {
    let start_time = page.get_body().iter().find_map(|line| {
        let line = get_content_of_raw_colored_text(line);
        parse_prompt_time(&line, 0)
    })?;
    let end_time = page.get_body().iter().rev().find_map(|line| {
        let line = get_content_of_raw_colored_text(line);
        parse_prompt_time(&line, start_time)
    })?;
    Some((start_time, end_time))
}
