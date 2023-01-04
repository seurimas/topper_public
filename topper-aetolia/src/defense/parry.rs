use crate::{classes::Class, db::*, observables::*, timeline::*, types::*};

pub fn get_needed_parry<DB: AetDatabaseModule + ?Sized>(
    timeline: &AetTimeline,
    me: &String,
    target: &String,
    strategy: &String,
    db: Option<&DB>,
) -> Option<LType> {
    if let Ok(parry) = get_preferred_parry(timeline, me, target, strategy, db) {
        let me = timeline.state.borrow_agent(me);
        if let Some(current) = me.parrying {
            if current == parry {
                None
            } else {
                Some(parry)
            }
        } else {
            Some(parry)
        }
    } else {
        None
    }
}

pub fn get_restore_parry(timeline: &AetTimeline, me: &String) -> Option<LType> {
    let me = timeline.state.borrow_agent(me);
    if let Some((restoring, _duration, _regenerating)) = me.get_restoring() {
        if restoring == LType::LeftLegDamage {
            Some(LType::RightLegDamage)
        } else if restoring == LType::RightLegDamage {
            Some(LType::LeftLegDamage)
        } else {
            None
        }
    } else {
        None
    }
}

pub fn get_worst_damage(timeline: &AetTimeline, me: &String) -> Option<LType> {
    let me = timeline.state.borrow_agent(me);
    let mut top_non_restoring = None;
    for limb in LIMBS.to_vec() {
        let limb_state = me.get_limb_state(limb);
        if let Some((top_damage, _top_limb)) = top_non_restoring {
            if !limb_state.is_restoring && limb_state.damage > top_damage {
                top_non_restoring = Some((limb_state.damage, limb));
            }
        } else if !limb_state.is_restoring && limb_state.damage > 8.0 {
            top_non_restoring = Some((limb_state.damage, limb));
        }
    }
    top_non_restoring.map(|top| top.1)
}

pub fn get_worst_bruise(timeline: &AetTimeline, me: &String) -> Option<LType> {
    let me = timeline.state.borrow_agent(me);
    let mut top_bruised = None;
    for limb in LIMBS.to_vec() {
        let limb_state = me.get_limb_state(limb);
        if let Some((top_bruise_level, _top_limb)) = top_bruised {
            if limb_state.bruise_level > top_bruise_level {
                top_bruised = Some((limb_state.bruise_level, limb));
            }
        } else if limb_state.bruise_level > 0 {
            top_bruised = Some((limb_state.bruise_level, limb));
        }
    }
    top_bruised.map(|top| top.1)
}

pub fn get_preferred_parry<DB: AetDatabaseModule + ?Sized>(
    timeline: &AetTimeline,
    me: &String,
    target: &String,
    strategy: &String,
    db: Option<&DB>,
) -> Result<LType, String> {
    if let Some(mut class) = db.and_then(|db| db.get_class(target)) {
        if class.is_mirror() {
            class = class.normal();
        }
        match class {
            Class::Shapeshifter => {
                let myself = timeline.state.borrow_agent(me);
                let limbs_state = myself.get_limbs_state();
                if limbs_state.left_leg.broken && !limbs_state.left_leg.damaged {
                    Ok(LType::LeftLegDamage)
                } else if limbs_state.right_leg.broken && !limbs_state.right_leg.damaged {
                    Ok(LType::RightLegDamage)
                } else if limbs_state.left_arm.broken && !limbs_state.left_arm.damaged {
                    Ok(LType::LeftArmDamage)
                } else if limbs_state.right_arm.broken && !limbs_state.right_arm.damaged {
                    Ok(LType::RightArmDamage)
                } else if let Some(parry) = get_restore_parry(timeline, me) {
                    Ok(parry)
                } else {
                    Ok(get_worst_damage(timeline, me).unwrap_or(LType::HeadDamage))
                }
            }
            Class::Zealot => {
                let them = timeline.state.borrow_agent(target);
                match them.channel_state {
                    ChannelState::Heelrush(limb, _) => Ok(limb),
                    _ => {
                        let myself = timeline.state.borrow_agent(me);
                        if myself.is(FType::Heatspear) {
                            Ok(LType::TorsoDamage)
                        } else if let Some(parry) = get_restore_parry(timeline, me) {
                            Ok(parry)
                        } else {
                            Ok(get_worst_damage(timeline, me).unwrap_or(LType::TorsoDamage))
                        }
                    }
                }
            }
            Class::Sentinel => {
                let myself = timeline.state.borrow_agent(me);
                if myself.is(FType::Heartflutter) {
                    Ok(LType::TorsoDamage)
                } else if !myself.is(FType::Impatience) {
                    Ok(LType::HeadDamage)
                } else if let Some(parry) = get_restore_parry(timeline, me) {
                    Ok(parry)
                } else {
                    Ok(get_worst_damage(timeline, me).unwrap_or(LType::HeadDamage))
                }
            }
            Class::Teradrim => {
                if let Some(bruised) = get_worst_bruise(timeline, me) {
                    Ok(bruised)
                } else if let Some(parry) = get_restore_parry(timeline, me) {
                    Ok(parry)
                } else {
                    Ok(get_worst_damage(timeline, me).unwrap_or(LType::TorsoDamage))
                }
            }
            Class::Wayfarer => {
                let myself = timeline.state.borrow_agent(me);
                let limbs_state = myself.get_limbs_state();
                if limbs_state.left_leg.damaged
                    || limbs_state.right_leg.damaged
                    || limbs_state.left_arm.damaged
                    || limbs_state.right_arm.damaged
                {
                    if limbs_state.head.damage > 20.0 {
                        Ok(LType::HeadDamage)
                    } else if let Some(parry) = get_restore_parry(timeline, me) {
                        Ok(parry)
                    } else {
                        Ok(get_worst_damage(timeline, me).unwrap_or(LType::HeadDamage))
                    }
                } else if let Some(parry) = get_restore_parry(timeline, me) {
                    Ok(parry)
                } else {
                    Ok(get_worst_damage(timeline, me).unwrap_or(LType::HeadDamage))
                }
            }
            _ => Ok(get_worst_damage(timeline, me).unwrap_or(LType::HeadDamage)),
        }
    } else {
        get_restore_parry(timeline, me)
            .or_else(|| get_worst_damage(timeline, me))
            .or_else(|| Some(LType::HeadDamage))
            .ok_or("Could not determine parry".to_string())
    }
}
