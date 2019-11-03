use crate::actions::*;
use crate::types::*;

pub fn dstab_action(
    (venom1, affliction1): (String, FType),
    (venom2, affliction2): (String, FType),
) -> StateAction {
    StateAction {
        name: format!("dstab {} {}", venom1, venom2),
        changes: vec![
            balance_change(BType::Balance, 2.8),
            flag_me(FType::Shield, false),
            afflict(affliction1),
            afflict(affliction2),
        ],
        initial: vec![
            alive(),
            target(alive()),
            target(lacks(FType::Rebounding)),
            target(lacks(FType::Shield)),
            has(BType::Balance),
            has(BType::Equil),
            target(lacks(affliction1)),
            target(lacks(affliction2)),
        ],
    }
}

pub fn flay_action() -> StateAction {
    let flayable = vec![FType::Shield, FType::Rebounding, FType::HardenedSkin];
    StateAction {
        name: "flay".into(),
        changes: vec![
            balance_change(BType::Balance, 2.5),
            flag_me(FType::Shield, false),
            target(cure_in_order(flayable.clone())),
        ],
        initial: vec![
            alive(),
            target(alive()),
            has(BType::Balance),
            has(BType::Equil),
            target(some(flayable)),
        ],
    }
}
