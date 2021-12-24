use crate::{battle_stats::get_aff_icon, log};
use strum_macros::Display;
use topper_aetolia::{
    curatives::*,
    types::{AgentState, FType, LType},
};
use yew::prelude::*;

const MAX_SALVES: usize = 10;

#[derive(Clone, Debug, Display, PartialEq)]
pub enum Salve {
    Epidermal,
    Caloric,
    Soothing,
    Mending,
    Restoration,
}

impl Salve {
    pub fn get_salve_affs(&self, limb: LType) -> Vec<FType> {
        match (self, limb) {
            (Salve::Epidermal, LType::HeadDamage) => EPIDERMAL_HEAD_ORDER.to_vec(),
            (Salve::Epidermal, LType::TorsoDamage) => EPIDERMAL_TORSO_ORDER.to_vec(),
            (Salve::Caloric, LType::TorsoDamage) => CALORIC_TORSO_ORDER.to_vec(),
            (Salve::Soothing, LType::TorsoDamage) => SOOTHING_SKIN_ORDER.to_vec(),
            (Salve::Mending, LType::HeadDamage) => MENDING_HEAD_ORDER.to_vec(),
            (Salve::Mending, LType::TorsoDamage) => MENDING_TORSO_ORDER.to_vec(),
            (Salve::Mending, LType::LeftArmDamage) => MENDING_LEFT_ARM_ORDER.to_vec(),
            (Salve::Mending, LType::RightArmDamage) => MENDING_RIGHT_ARM_ORDER.to_vec(),
            (Salve::Mending, LType::LeftLegDamage) => MENDING_LEFT_LEG_ORDER.to_vec(),
            (Salve::Mending, LType::RightLegDamage) => MENDING_RIGHT_LEG_ORDER.to_vec(),
            (Salve::Restoration, LType::HeadDamage) => RESTORATION_HEAD_ORDER.to_vec(),
            (Salve::Restoration, LType::TorsoDamage) => RESTORATION_TORSO_ORDER.to_vec(),
            (Salve::Restoration, LType::LeftArmDamage) => RESTORATION_LEFT_ARM_ORDER.to_vec(),
            (Salve::Restoration, LType::RightArmDamage) => RESTORATION_RIGHT_ARM_ORDER.to_vec(),
            (Salve::Restoration, LType::LeftLegDamage) => RESTORATION_LEFT_LEG_ORDER.to_vec(),
            (Salve::Restoration, LType::RightLegDamage) => RESTORATION_RIGHT_LEG_ORDER.to_vec(),
            _ => panic!("No salves for {} {:?}", self, limb),
        }
    }

    pub fn get_aff_states(&self, limb: LType, who: &AgentState) -> [bool; MAX_SALVES] {
        let mut aff_states = [false; MAX_SALVES];
        for (aff_idx, aff) in self.get_salve_affs(limb).iter().enumerate() {
            aff_states[aff_idx] = who.is(*aff);
        }
        aff_states
    }
}

#[derive(Clone, Properties, PartialEq)]
pub struct SalveStripProps {
    pub salve: Salve,
    pub limb: LType,
    pub aff_states: [bool; MAX_SALVES],
}

pub struct SalveStrip;

impl Component for SalveStrip {
    type Properties = SalveStripProps;

    type Message = ();

    fn create(ctx: &Context<Self>) -> Self {
        SalveStrip
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let salve_affs = ctx
            .props()
            .salve
            .get_salve_affs(ctx
                .props().limb)
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
            <div class={format!("salve_strip salve_strip--{}_{:?}", ctx.props().salve, ctx.props().limb)}>
              {salve_affs}
            </div>
        }
    }
}
