use serde::{Deserialize, Serialize};
use topper_aetolia::{timeline::AetTimelineState, types::FType};
use yew::prelude::*;

use super::page::ExplainerPageMessage;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Mutation {
    AddAffliction(String, FType),
    RemoveAffliction(String, FType),
}

pub struct StateBlock;

#[derive(Properties)]
pub struct StateBlockProperties {
    pub state: AetTimelineState,
    pub me: String,
    pub you: String,
}

impl PartialEq for StateBlockProperties {
    fn eq(&self, other: &Self) -> bool {
        self.state.time == other.state.time
    }
}

impl Component for StateBlock {
    type Message = ExplainerPageMessage;
    type Properties = StateBlockProperties;

    fn create(ctx: &Context<Self>) -> Self {
        Self
    }
    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();
        html!(<div class="page__state">{format!("State of {} v {}", props.me, props.you)}</div>)
    }
}
