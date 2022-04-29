use crate::{battle_stats::get_aff_icon, log};
use strum_macros::Display;
use topper_aetolia::{
    curatives::*,
    types::{AgentState, FType},
};
use yew::prelude::*;

#[derive(Clone, Debug, Display, PartialEq)]
pub enum Pill {
    Antipsychotic,
    Euphoriant,
    Eucrasia,
    Decongestant,
    Depressant,
    Coagulation,
    Steroid,
    Opiate,
    Panacea,
}

impl Pill {
    pub fn get_pill_affs(&self) -> Vec<FType> {
        match self {
            Pill::Antipsychotic => ANTIPSYCHOTIC_ORDER.to_vec(),
            Pill::Euphoriant => EUPHORIANT_ORDER.to_vec(),
            Pill::Eucrasia => EUCRASIA_ORDER.to_vec(),
            Pill::Decongestant => DECONGESTANT_ORDER.to_vec(),
            Pill::Depressant => DEPRESSANT_ORDER.to_vec(),
            Pill::Coagulation => COAGULATION_ORDER.to_vec(),
            Pill::Steroid => STEROID_ORDER.to_vec(),
            Pill::Opiate => OPIATE_ORDER.to_vec(),
            Pill::Panacea => PANACEA_ORDER.to_vec(),
        }
    }

    pub fn get_aff_states(&self, who: &AgentState) -> [bool; 16] {
        let mut aff_states = [false; 16];
        for (aff_idx, aff) in self.get_pill_affs().iter().enumerate() {
            aff_states[aff_idx] = who.is(*aff);
        }
        aff_states
    }
}

#[derive(Clone, Properties, PartialEq)]
pub struct PillStripProps {
    pub pill: Pill,
    pub aff_states: [bool; 16],
}

pub struct PillStrip;

impl Component for PillStrip {
    type Properties = PillStripProps;

    type Message = ();

    fn create(ctx: &Context<Self>) -> Self {
        PillStrip
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let pill_affs = ctx
            .props()
            .pill
            .get_pill_affs()
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
            <div class={format!("pill_strip pill_strip--{}", ctx.props().pill)}>
              {pill_affs}
            </div>
        }
    }
}
