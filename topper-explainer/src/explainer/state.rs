use serde::{Deserialize, Serialize};
use topper_aetolia::{curatives::MENTAL_AFFLICTIONS, timeline::AetTimelineState, types::*};
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
        let me = props.state.borrow_agent(&props.me);
        let you = props.state.borrow_agent(&props.you);
        html!(<div class="page__state">
            <PlayerState state={me} />
            <PlayerState state={you} />
        </div>)
    }
}

struct PlayerState;

#[derive(Properties, PartialEq)]
pub struct PlayerStateProperties {
    pub state: AgentState,
}

impl Component for PlayerState {
    type Message = ();
    type Properties = PlayerStateProperties;

    fn create(ctx: &Context<Self>) -> Self {
        Self
    }
    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();
        let state = &props.state;
        let class = state.class_state.get_normalized_class();
        let aff_count = state.aff_count();
        let afflictions = state.flags.aff_iter().collect::<Vec<FType>>();
        let tree = state.get_balance(BType::Tree);
        let focus = state.get_balance(BType::Focus);
        let fitness = state.get_balance(BType::Fitness);
        let class_cure = state.get_balance(BType::ClassCure1);
        let rebounding = if state.is(FType::Rebounding) {
            None
        } else {
            Some(state.get_balance(BType::Rebounding))
        };
        let limbs = vec![
            LType::HeadDamage,
            LType::TorsoDamage,
            LType::LeftArmDamage,
            LType::RightArmDamage,
            LType::LeftLegDamage,
            LType::RightLegDamage,
        ]
        .iter()
        .map(|limb| (limb.to_string(), state.get_limb_state(*limb)))
        .collect::<Vec<(String, LimbState)>>();

        html!(<div class="page__state__player">
          <span class="page__state__player__class">{"Class: "}{class.map_or("Unknown".to_string(), |class| class.to_string())}</span>
          <span class="page__state__player__aff_count">{"Afflictions: "}{aff_count}</span>
          <BalanceIndicator name="tree" balance={tree} />
          <BalanceIndicator name="focus" balance={focus} />
          <BalanceIndicator name="fitness" balance={fitness} />
          <BalanceIndicator name="class_cure" balance={class_cure} />
          <ReboundingIndicator balance={rebounding} />
          <LimbsIndicator limbs={limbs} />
          <AfflictionsIndicator afflictions={afflictions} />
        </div>)
    }
}

#[derive(Properties, PartialEq)]
pub struct BalanceIndicatorProps {
    pub name: String,
    pub balance: f32,
}

#[function_component]
fn BalanceIndicator(props: &BalanceIndicatorProps) -> Html {
    let icon = match props.name.as_str() {
        "tree" => "🌳",
        "focus" => "🧠",
        "fitness" => "😤",
        _ => "🔮",
    };
    html!(<div class="page__state__player__balance" title={props.name.clone()}>
        <span>{icon}{": "}</span>
        <span>{if props.balance > 0. {
            props.balance.to_string()
        } else {
            "Ready".to_string()
        }}</span>
    </div>)
}

#[derive(Properties, PartialEq)]
pub struct ReboundingIndicatorProps {
    pub balance: Option<f32>,
}

#[function_component]
fn ReboundingIndicator(props: &ReboundingIndicatorProps) -> Html {
    html!(<div class="page__state__player__rebounding">
        <span>{"Rebounding: "}</span>
        <span>{if let Some(balance) = props.balance {
            if balance > 0. {
                html!({balance})
            } else {
                html!({"XX"})
            }
        } else {
            html!({"UP"})
        }}</span>
    </div>)
}

#[derive(Properties, PartialEq)]
pub struct LimbsIndicatorProps {
    pub limbs: Vec<(String, LimbState)>,
}

#[function_component]
fn LimbsIndicator(props: &LimbsIndicatorProps) -> Html {
    let limb_indicator = props.limbs.iter().map(|(name, state)| {
        let damage_id = (state.damage / 10.) as i32;
        let broken_state = if state.amputated {
            "amputated"
        } else if state.mangled {
            "mangled"
        } else if state.damaged {
            "damaged"
        } else if state.broken {
            "broken"
        } else {
            "healthy"
        };
        let restoring = state.is_restoring;
        let bruise_level = state.bruise_level;
        let classes = classes!(
            "limb",
            format!("limb--damaged_{}", damage_id),
            format!("limb--broken_{}", broken_state),
            if restoring {
                Some("limb--restoring")
            } else {
                None
            },
            format!("limb--bruise_level_{}", bruise_level),
        );
        html!(<div
            class={classes}
            title={format!("{} and damage level {}", broken_state, damage_id)}
        >
            {name}
        </div>)
    });
    html!(<div class="page__state__player__limbs">
      {for limb_indicator}
    </div>)
}

#[derive(Properties, PartialEq)]
pub struct AfflictionsIndicatorProps {
    pub afflictions: Vec<FType>,
}

#[function_component]
fn AfflictionsIndicator(props: &AfflictionsIndicatorProps) -> Html {
    let aff_indicator = props.afflictions.iter().map(|aff| {
        let classes = classes!(
            "aff",
            format!("aff--{}", aff.to_string()),
            if MENTAL_AFFLICTIONS.contains(aff) {
                Some("aff--mental")
            } else {
                None
            },
        );
        html!(<div class={classes}>{aff.to_name()}</div>)
    });
    html!(<div class="page__state__player__affs">
      {for aff_indicator}
    </div>)
}
