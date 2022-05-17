use crate::log;
use topper_aetolia::{
    timeline::AetTimeline,
    types::{AgentState, FType, LType},
};
use yew::prelude::*;
mod pill_strip;
mod salve_strip;
mod smoke_strip;
use pill_strip::*;
use salve_strip::*;
use smoke_strip::*;

type BattleStats = AgentState;

#[derive(Clone, Properties, PartialEq, Debug)]
pub struct BattleStatsProps {
    pub me: bool,
    pub battle_stats: BattleStats,
}

pub struct BattleStatsElem;

impl Component for BattleStatsElem {
    type Properties = BattleStatsProps;

    type Message = ();

    fn create(_ctx: &Context<Self>) -> Self {
        BattleStatsElem
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        log(format!("{:?}", ctx.props()).as_ref());
        html! {
            <div class={if ctx.props().me { "battle_stats battle_stats--me"} else { "battle_stats battle_stats--you" }}>
                <div class="pills">
                    <PillStrip pill={Pill::Antipsychotic} aff_states={Pill::Antipsychotic.get_aff_states(&ctx.props().battle_stats)} />
                    <PillStrip pill={Pill::Euphoriant} aff_states={Pill::Euphoriant.get_aff_states(&ctx.props().battle_stats)} />
                    <PillStrip pill={Pill::Decongestant} aff_states={Pill::Decongestant.get_aff_states(&ctx.props().battle_stats)} />
                    <PillStrip pill={Pill::Depressant} aff_states={Pill::Depressant.get_aff_states(&ctx.props().battle_stats)} />
                    <PillStrip pill={Pill::Coagulation} aff_states={Pill::Coagulation.get_aff_states(&ctx.props().battle_stats)} />
                    <PillStrip pill={Pill::Eucrasia} aff_states={Pill::Eucrasia.get_aff_states(&ctx.props().battle_stats)} />
                    <PillStrip pill={Pill::Opiate} aff_states={Pill::Opiate.get_aff_states(&ctx.props().battle_stats)} />
                    <PillStrip pill={Pill::Steroid} aff_states={Pill::Steroid.get_aff_states(&ctx.props().battle_stats)} />
                    <PillStrip pill={Pill::Panacea} aff_states={Pill::Panacea.get_aff_states(&ctx.props().battle_stats)} />
                </div>
                <div class="smokes">
                    <SmokeStrip smoke={Smoke::Yarrow} aff_states={Smoke::Yarrow.get_aff_states(&ctx.props().battle_stats)} />
                    <SmokeStrip smoke={Smoke::Willow} aff_states={Smoke::Willow.get_aff_states(&ctx.props().battle_stats)} />
                    <SmokeStrip smoke={Smoke::Reishi} aff_states={Smoke::Reishi.get_aff_states(&ctx.props().battle_stats)} />
                </div>
                <div class="salves">
                    <SalveStrip salve={Salve::Epidermal} limb={LType::HeadDamage} aff_states={Salve::Epidermal.get_aff_states(LType::HeadDamage, &ctx.props().battle_stats)} />
                    <SalveStrip salve={Salve::Epidermal} limb={LType::TorsoDamage} aff_states={Salve::Epidermal.get_aff_states(LType::TorsoDamage, &ctx.props().battle_stats)} />
                    <SalveStrip salve={Salve::Caloric} limb={LType::TorsoDamage} aff_states={Salve::Caloric.get_aff_states(LType::TorsoDamage, &ctx.props().battle_stats)} />
                    <SalveStrip salve={Salve::Soothing} limb={LType::TorsoDamage} aff_states={Salve::Soothing.get_aff_states(LType::TorsoDamage, &ctx.props().battle_stats)} />
                    <SalveStrip salve={Salve::Mending} limb={LType::HeadDamage} aff_states={Salve::Mending.get_aff_states(LType::HeadDamage, &ctx.props().battle_stats)} />
                    <SalveStrip salve={Salve::Mending} limb={LType::TorsoDamage} aff_states={Salve::Mending.get_aff_states(LType::TorsoDamage, &ctx.props().battle_stats)} />
                    <SalveStrip salve={Salve::Mending} limb={LType::LeftArmDamage} aff_states={Salve::Mending.get_aff_states(LType::LeftArmDamage, &ctx.props().battle_stats)} />
                    <SalveStrip salve={Salve::Mending} limb={LType::RightArmDamage} aff_states={Salve::Mending.get_aff_states(LType::RightArmDamage, &ctx.props().battle_stats)} />
                    <SalveStrip salve={Salve::Mending} limb={LType::LeftLegDamage} aff_states={Salve::Mending.get_aff_states(LType::LeftLegDamage, &ctx.props().battle_stats)} />
                    <SalveStrip salve={Salve::Mending} limb={LType::RightLegDamage} aff_states={Salve::Mending.get_aff_states(LType::RightLegDamage, &ctx.props().battle_stats)} />
                    <SalveStrip salve={Salve::Restoration} limb={LType::HeadDamage} aff_states={Salve::Restoration.get_aff_states(LType::HeadDamage, &ctx.props().battle_stats)} />
                    <SalveStrip salve={Salve::Restoration} limb={LType::TorsoDamage} aff_states={Salve::Restoration.get_aff_states(LType::TorsoDamage, &ctx.props().battle_stats)} />
                    <SalveStrip salve={Salve::Restoration} limb={LType::LeftArmDamage} aff_states={Salve::Restoration.get_aff_states(LType::LeftArmDamage, &ctx.props().battle_stats)} />
                    <SalveStrip salve={Salve::Restoration} limb={LType::RightArmDamage} aff_states={Salve::Restoration.get_aff_states(LType::RightArmDamage, &ctx.props().battle_stats)} />
                    <SalveStrip salve={Salve::Restoration} limb={LType::LeftLegDamage} aff_states={Salve::Restoration.get_aff_states(LType::LeftLegDamage, &ctx.props().battle_stats)} />
                    <SalveStrip salve={Salve::Restoration} limb={LType::RightLegDamage} aff_states={Salve::Restoration.get_aff_states(LType::RightLegDamage, &ctx.props().battle_stats)} />
                </div>
            </div>
        }
    }
}

pub fn get_battle_stats(timeline: &AetTimeline, who: &String) -> BattleStats {
    let state = timeline.state.borrow_agent(who);
    log(format!("{} {:?}", who, state).as_ref());
    state
}

pub fn get_aff_icon(aff: &FType) -> String {
    let mut icon = format!("{}", aff);
    icon.truncate(1);
    icon
}
