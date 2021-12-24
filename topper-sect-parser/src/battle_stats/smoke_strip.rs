use crate::{battle_stats::get_aff_icon, log};
use strum_macros::Display;
use topper_aetolia::{
    curatives::*,
    types::{AgentState, FType},
};
use yew::prelude::*;

const MAX_SMOKES: usize = 5;

#[derive(Clone, Debug, Display, PartialEq)]
pub enum Smoke {
    Yarrow,
    Willow,
    Reishi,
}

impl Smoke {
    pub fn get_smoke_affs(&self) -> Vec<FType> {
        match self {
            Smoke::Yarrow => YARROW_ORDER.to_vec(),
            Smoke::Willow => WILLOW_ORDER.to_vec(),
            Smoke::Reishi => vec![FType::Rebounding].to_vec(),
        }
    }

    pub fn get_aff_states(&self, who: &AgentState) -> [bool; MAX_SMOKES] {
        let mut aff_states = [false; MAX_SMOKES];
        for (aff_idx, aff) in self.get_smoke_affs().iter().enumerate() {
            aff_states[aff_idx] = who.is(*aff);
        }
        aff_states
    }
}

#[derive(Clone, Properties, PartialEq)]
pub struct SmokeStripProps {
    pub smoke: Smoke,
    pub aff_states: [bool; MAX_SMOKES],
}

pub struct SmokeStrip;

impl Component for SmokeStrip {
    type Properties = SmokeStripProps;

    type Message = ();

    fn create(ctx: &Context<Self>) -> Self {
        SmokeStrip
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let smoke_affs = ctx
            .props()
            .smoke
            .get_smoke_affs()
            .iter()
            .enumerate()
            .map(|(aff_idx, aff)| {
                let afflicted_str = if ctx.props().aff_states[aff_idx] {
                    "afflicted"
                } else {
                    "cured"
                };
                html! {
                    <div class={format!("aff aff--{} aff--{}", afflicted_str, aff)} title={aff.to_string()}>{get_aff_icon(aff)}</div>
                }
            })
            .collect::<Html>();
        html! {
            <div class={format!("smoke_strip smoke_strip--{}", ctx.props().smoke)}>
              {smoke_affs}
            </div>
        }
    }
}
