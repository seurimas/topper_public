use crate::{db::*, defense::*, observables::*, timeline::*, types::*};

pub fn get_action_plan(
    timeline: &AetTimeline,
    me: &String,
    target: &String,
    strategy: &String,
    db: Option<&impl AetDatabaseModule>,
) -> ActionPlan {
    let mut action_plan = ActionPlan::new(me);
    if should_regenerate(&timeline, me) {
        // balance = Box::new(RegenerateAction::new(me.to_string()));
    }
    if let Some(parry) = get_needed_parry(timeline, me, target, strategy, db) {
        // balance = Box::new(SeparatorAction::pair(
        //     Box::new(ParryAction::new(me.to_string(), parry)),
        //     balance,
        // ));
    }

    let me = timeline.state.borrow_agent(me);
    for pipe_refill in get_needed_refills(&me) {
        action_plan.add_to_front_of_qeb(Box::new(pipe_refill));
    }
    action_plan
}

pub fn get_attack(
    timeline: &AetTimeline,
    target: &String,
    strategy: &String,
    db: Option<&impl AetDatabaseModule>,
) -> String {
    let action_plan = get_action_plan(&timeline, &timeline.who_am_i(), &target, &strategy, db);
    action_plan.get_inputs(&timeline)
}
