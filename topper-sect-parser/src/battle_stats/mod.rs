use topper_aetolia::{timeline::AetTimeline, types::AgentState};
use yew::prelude::*;
mod pill_strip;
use crate::log;
use pill_strip::*;

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
                <PillStrip pill={Pill::Antipsychotic} aff_states={Pill::Antipsychotic.get_aff_states(&ctx.props().battle_stats)} />
                <PillStrip pill={Pill::Euphoriant} aff_states={Pill::Euphoriant.get_aff_states(&ctx.props().battle_stats)} />
                <PillStrip pill={Pill::Decongestant} aff_states={Pill::Decongestant.get_aff_states(&ctx.props().battle_stats)} />
                <PillStrip pill={Pill::Depressant} aff_states={Pill::Depressant.get_aff_states(&ctx.props().battle_stats)} />
                <PillStrip pill={Pill::Coagulation} aff_states={Pill::Coagulation.get_aff_states(&ctx.props().battle_stats)} />
                <PillStrip pill={Pill::Opiate} aff_states={Pill::Opiate.get_aff_states(&ctx.props().battle_stats)} />
                <PillStrip pill={Pill::Steroid} aff_states={Pill::Steroid.get_aff_states(&ctx.props().battle_stats)} />
                <PillStrip pill={Pill::Panacea} aff_states={Pill::Panacea.get_aff_states(&ctx.props().battle_stats)} />
            </div>
        }
    }
}

pub fn get_battle_stats(timeline: &AetTimeline, who: &String) -> BattleStats {
    let state = timeline.state.borrow_agent(who);
    log(format!("{} {:?}", who, state).as_ref());
    state
}
