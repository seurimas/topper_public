use serde::*;

use crate::{classes::group::*, observables::*, timeline::*, types::*};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Copy)]
pub enum Weavable {
    Nullstone,
    Boundary,
    Horologe,
    Goblet,
    Anelace,
    Thurible,
}

pub struct WeavingAction {
    pub caster: String,
    pub weaved: Weavable,
}

impl WeavingAction {
    pub fn new(caster: String, weaved: Weavable) -> Self {
        WeavingAction { caster, weaved }
    }
    pub fn nullstone(caster: String) -> Self {
        WeavingAction {
            caster,
            weaved: Weavable::Nullstone,
        }
    }
    pub fn boundary(caster: String) -> Self {
        WeavingAction {
            caster,
            weaved: Weavable::Boundary,
        }
    }
    pub fn horologe(caster: String) -> Self {
        WeavingAction {
            caster,
            weaved: Weavable::Horologe,
        }
    }
    pub fn goblet(caster: String) -> Self {
        WeavingAction {
            caster,
            weaved: Weavable::Goblet,
        }
    }
    pub fn anelace(caster: String) -> Self {
        WeavingAction {
            caster,
            weaved: Weavable::Anelace,
        }
    }
    pub fn thurible(caster: String) -> Self {
        WeavingAction {
            caster,
            weaved: Weavable::Thurible,
        }
    }

    pub fn get_skill(&self) -> &str {
        match self.weaved {
            Weavable::Anelace => "Anelace",
            Weavable::Boundary => "Boundary",
            Weavable::Goblet => "Goblet",
            Weavable::Horologe => "Horologe",
            Weavable::Nullstone => "Nullstone",
            Weavable::Thurible => "Thurible",
        }
    }
}

impl ActiveTransition for WeavingAction {
    fn simulate(&self, timeline: &AetTimeline) -> Vec<ProbableEvent> {
        let mut observations = vec![CombatAction::observation(
            &self.caster,
            &"Weaving",
            self.get_skill(),
            &"",
            &"",
        )];
        ProbableEvent::certain(observations)
    }

    fn act(&self, timeline: &AetTimeline) -> ActivateResult {
        Ok(match &self.weaved {
            Weavable::Anelace => "weave anelace".to_string(),
            Weavable::Boundary => "weave boundary".to_string(),
            Weavable::Goblet => "weave goblet".to_string(),
            Weavable::Horologe => "weave horologe".to_string(),
            Weavable::Nullstone => "weave nullstone".to_string(),
            Weavable::Thurible => "weave thurible".to_string(),
        })
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Copy)]
pub enum WeavingAttack {
    Tearing,
    Patchwork,
    Soundblast,
    Globes,
    Swindle,
    Barbs,
    Polarity,
    Effigy,
    Runeband,
    Bladestorm,
    Ironcollar,
    Headstitch,
    Heartcage,
}

pub struct WeavingAttackAction {
    pub caster: String,
    pub target: String,
    pub attack: WeavingAttack,
}

impl WeavingAttackAction {
    pub fn new(caster: String, target: String, attack: WeavingAttack) -> Self {
        WeavingAttackAction {
            caster,
            target,
            attack,
        }
    }
    pub fn barbs(caster: String, target: String) -> Self {
        WeavingAttackAction {
            caster,
            target,
            attack: WeavingAttack::Barbs,
        }
    }
    pub fn bladestorm(caster: String, target: String) -> Self {
        WeavingAttackAction {
            caster,
            target,
            attack: WeavingAttack::Bladestorm,
        }
    }
    pub fn effigy(caster: String, target: String) -> Self {
        WeavingAttackAction {
            caster,
            target,
            attack: WeavingAttack::Effigy,
        }
    }
    pub fn globes(caster: String, target: String) -> Self {
        WeavingAttackAction {
            caster,
            target,
            attack: WeavingAttack::Globes,
        }
    }
    pub fn headstitch(caster: String, target: String) -> Self {
        WeavingAttackAction {
            caster,
            target,
            attack: WeavingAttack::Headstitch,
        }
    }
    pub fn heartcage(caster: String, target: String) -> Self {
        WeavingAttackAction {
            caster,
            target,
            attack: WeavingAttack::Heartcage,
        }
    }
    pub fn ironcollar(caster: String, target: String) -> Self {
        WeavingAttackAction {
            caster,
            target,
            attack: WeavingAttack::Ironcollar,
        }
    }
    pub fn polarity(caster: String, target: String) -> Self {
        WeavingAttackAction {
            caster,
            target,
            attack: WeavingAttack::Polarity,
        }
    }
    pub fn runeband(caster: String, target: String) -> Self {
        WeavingAttackAction {
            caster,
            target,
            attack: WeavingAttack::Runeband,
        }
    }
    pub fn soundblast(caster: String, target: String) -> Self {
        WeavingAttackAction {
            caster,
            target,
            attack: WeavingAttack::Soundblast,
        }
    }
    pub fn swindle(caster: String, target: String) -> Self {
        WeavingAttackAction {
            caster,
            target,
            attack: WeavingAttack::Swindle,
        }
    }
    pub fn tearing(caster: String, target: String) -> Self {
        WeavingAttackAction {
            caster,
            target,
            attack: WeavingAttack::Tearing,
        }
    }
    pub fn patchwork(caster: String, target: String) -> Self {
        WeavingAttackAction {
            caster,
            target,
            attack: WeavingAttack::Patchwork,
        }
    }

    pub fn get_skill(&self) -> &str {
        match self.attack {
            WeavingAttack::Tearing => "Tearing",
            WeavingAttack::Patchwork => "Patchwork",
            WeavingAttack::Soundblast => "Soundblast",
            WeavingAttack::Globes => "Globes",
            WeavingAttack::Swindle => "Swindle",
            WeavingAttack::Barbs => "Barbs",
            WeavingAttack::Polarity => "Polarity",
            WeavingAttack::Effigy => "Effigy",
            WeavingAttack::Runeband => "Runeband",
            WeavingAttack::Bladestorm => "Bladestorm",
            WeavingAttack::Ironcollar => "Ironcollar",
            WeavingAttack::Headstitch => "Headstitch",
            WeavingAttack::Heartcage => "Heartcage",
        }
    }
}

impl ActiveTransition for WeavingAttackAction {
    fn simulate(&self, timeline: &AetTimeline) -> Vec<ProbableEvent> {
        let mut observations = vec![CombatAction::observation(
            &self.caster,
            &"Weaving",
            self.get_skill(),
            &"",
            &self.target,
        )];
        ProbableEvent::certain(observations)
    }

    fn act(&self, timeline: &AetTimeline) -> ActivateResult {
        Ok(match &self.attack {
            WeavingAttack::Tearing => format!("weave tearing {}", self.target),
            WeavingAttack::Patchwork => format!("weave patchwork {}", self.target),
            WeavingAttack::Soundblast => format!("weave soundblast {}", self.target),
            WeavingAttack::Globes => format!("weave globes {}", self.target),
            WeavingAttack::Swindle => format!("weave swindle {}", self.target),
            WeavingAttack::Barbs => format!("weave barbs {}", self.target),
            WeavingAttack::Polarity => format!("weave polarity {}", self.target),
            WeavingAttack::Effigy => format!("weave effigy {}", self.target),
            WeavingAttack::Runeband => format!("weave runeband {}", self.target),
            WeavingAttack::Bladestorm => format!("weave bladestorm {}", self.target),
            WeavingAttack::Ironcollar => format!("weave ironcollar {}", self.target),
            WeavingAttack::Headstitch => format!("weave headstitch {}", self.target),
            WeavingAttack::Heartcage => format!("weave heartcage {}", self.target),
        })
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum PerformanceAttack {
    TempoOne(String),
    TempoTwo(String, String),
    TempoThree(String, String, String),
    Needle(String),
    Harry(String),
    Bravado(String),
    Pierce,
    Seduce,
    Guilt,
    Ridicule,
    Crackshot,
    Quip,
    Sock,
    Hiltblow,
    Cadence,
}

impl PerformanceAttack {
    pub fn needs_weapon(&self) -> bool {
        match self {
            Self::TempoOne(_)
            | Self::TempoTwo(_, _)
            | Self::TempoThree(_, _, _)
            | Self::Harry(_)
            | Self::Bravado(_)
            | Self::Cadence
            | Self::Hiltblow => true,
            _ => false,
        }
    }

    pub fn gets_rebounded(&self) -> bool {
        match self {
            Self::TempoOne(_)
            | Self::TempoTwo(_, _)
            | Self::TempoThree(_, _, _)
            | Self::Harry(_)
            | Self::Bravado(_)
            | Self::Cadence
            | Self::Hiltblow => true,
            _ => false,
        }
    }
}

pub struct PerformanceAttackAction {
    pub caster: String,
    pub target: String,
    pub attack: PerformanceAttack,
}

impl PerformanceAttackAction {
    pub fn new(caster: String, target: String, attack: PerformanceAttack) -> Self {
        PerformanceAttackAction {
            caster,
            target,
            attack,
        }
    }
    pub fn needle(caster: String, target: String, venom: String) -> Self {
        PerformanceAttackAction {
            caster,
            target,
            attack: PerformanceAttack::Needle(venom),
        }
    }
    pub fn harry(caster: String, target: String, venom: String) -> Self {
        PerformanceAttackAction {
            caster,
            target,
            attack: PerformanceAttack::Harry(venom),
        }
    }
    pub fn bravado(caster: String, target: String, venom: String) -> Self {
        PerformanceAttackAction {
            caster,
            target,
            attack: PerformanceAttack::Bravado(venom),
        }
    }
    pub fn tempo_one(caster: String, target: String, venom: String) -> Self {
        PerformanceAttackAction {
            caster,
            target,
            attack: PerformanceAttack::TempoOne(venom),
        }
    }
    pub fn tempo_two(caster: String, target: String, venom_one: String, venom_two: String) -> Self {
        PerformanceAttackAction {
            caster,
            target,
            attack: PerformanceAttack::TempoTwo(venom_one, venom_two),
        }
    }
    pub fn tempo_three(
        caster: String,
        target: String,
        venom_one: String,
        venom_two: String,
        venom_three: String,
    ) -> Self {
        PerformanceAttackAction {
            caster,
            target,
            attack: PerformanceAttack::TempoThree(venom_one, venom_two, venom_three),
        }
    }
    pub fn cadence(caster: String, target: String) -> Self {
        PerformanceAttackAction {
            caster,
            target,
            attack: PerformanceAttack::Cadence,
        }
    }
    pub fn crackshot(caster: String, target: String) -> Self {
        PerformanceAttackAction {
            caster,
            target,
            attack: PerformanceAttack::Crackshot,
        }
    }
    pub fn guilt(caster: String, target: String) -> Self {
        PerformanceAttackAction {
            caster,
            target,
            attack: PerformanceAttack::Guilt,
        }
    }
    pub fn hiltblow(caster: String, target: String) -> Self {
        PerformanceAttackAction {
            caster,
            target,
            attack: PerformanceAttack::Hiltblow,
        }
    }
    pub fn pierce(caster: String, target: String) -> Self {
        PerformanceAttackAction {
            caster,
            target,
            attack: PerformanceAttack::Pierce,
        }
    }
    pub fn quip(caster: String, target: String) -> Self {
        PerformanceAttackAction {
            caster,
            target,
            attack: PerformanceAttack::Quip,
        }
    }
    pub fn ridicule(caster: String, target: String) -> Self {
        PerformanceAttackAction {
            caster,
            target,
            attack: PerformanceAttack::Ridicule,
        }
    }
    pub fn seduce(caster: String, target: String) -> Self {
        PerformanceAttackAction {
            caster,
            target,
            attack: PerformanceAttack::Seduce,
        }
    }
    pub fn sock(caster: String, target: String) -> Self {
        PerformanceAttackAction {
            caster,
            target,
            attack: PerformanceAttack::Sock,
        }
    }
}

impl ActiveTransition for PerformanceAttackAction {
    fn simulate(&self, timeline: &AetTimeline) -> Vec<ProbableEvent> {
        vec![]
    }
    fn act(&self, timeline: &AetTimeline) -> ActivateResult {
        let action = match &self.attack {
            PerformanceAttack::TempoOne(venom) => format!("tempo {} {}", self.target, venom),
            PerformanceAttack::TempoTwo(venom_one, venom_two) => format!(
                "tempo {} {};;envenom falchion with {}",
                self.target, venom_one, venom_two
            ),
            PerformanceAttack::TempoThree(venom_one, venom_two, venom_three) => format!(
                "tempo {} {};;envenom falchion with {};;envenom falchion with {}",
                self.target, venom_one, venom_three, venom_two
            ),
            PerformanceAttack::Needle(venom) => format!("needle {} {}", self.target, venom),
            PerformanceAttack::Harry(venom) => format!("harry {} {}", self.target, venom),
            PerformanceAttack::Bravado(venom) => format!("bravado {} {}", self.target, venom),
            PerformanceAttack::Pierce => format!("pierce {}", self.target),
            PerformanceAttack::Seduce => format!("seduce {}", self.target),
            PerformanceAttack::Guilt => format!("guilt {}", self.target),
            PerformanceAttack::Ridicule => format!("ridicule {}", self.target),
            PerformanceAttack::Crackshot => format!("crackshot {}", self.target),
            PerformanceAttack::Quip => format!("quip {}", self.target),
            PerformanceAttack::Sock => format!("sock {}", self.target),
            PerformanceAttack::Hiltblow => format!("hiltblow {}", self.target),
            PerformanceAttack::Cadence => format!("cadence {}", self.target),
        };
        if should_call_venoms(timeline) {
            let called = match &self.attack {
                PerformanceAttack::TempoOne(venom) => {
                    call_venom(&self.target, venom, Some("Rhythm"))
                }
                PerformanceAttack::TempoTwo(venom_one, venom_two) => {
                    call_venoms(&self.target, venom_one, venom_two, Some("Rhythm"))
                }

                PerformanceAttack::TempoThree(venom_one, venom_two, venom_three) => {
                    call_triple_venoms(
                        &self.target,
                        venom_one,
                        venom_two,
                        venom_three,
                        Some("Rhythm"),
                    )
                }
                PerformanceAttack::Needle(venom) => call_venom(&self.target, venom, None),
                PerformanceAttack::Harry(venom) => call_venom(&self.target, venom, None),
                PerformanceAttack::Bravado(venom) => call_venom(&self.target, venom, None),
                _ => "".to_string(),
            };
            Ok(format!("{};;{}", called, action))
        } else {
            Ok(action)
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct SongAction {
    pub played: bool,
    pub caster: String,
    pub song: Song,
}

impl SongAction {
    pub fn sing(caster: String, song: Song) -> Self {
        SongAction {
            played: false,
            caster,
            song,
        }
    }
    pub fn play(caster: String, song: Song) -> Self {
        SongAction {
            played: true,
            caster,
            song,
        }
    }
}

impl ActiveTransition for SongAction {
    fn simulate(&self, timeline: &AetTimeline) -> Vec<ProbableEvent> {
        vec![]
    }
    fn act(&self, timeline: &AetTimeline) -> ActivateResult {
        Ok(if self.played {
            format!("play song of {}", self.song)
        } else {
            format!("sing song of {}", self.song)
        })
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct AnelaceAction {
    pub caster: String,
    pub target: String,
}

impl AnelaceAction {
    pub fn new(caster: String, target: String) -> Self {
        AnelaceAction { caster, target }
    }
}

impl ActiveTransition for AnelaceAction {
    fn simulate(&self, timeline: &AetTimeline) -> Vec<ProbableEvent> {
        ProbableEvent::certain(vec![CombatAction::observation(
            &self.caster,
            "Weaving",
            "Anelace",
            "stab",
            &self.target,
        )])
    }
    fn act(&self, timeline: &AetTimeline) -> ActivateResult {
        Ok(format!("stab {}", self.target))
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct ColdReadAction {
    pub caster: String,
    pub target: String,
}

impl ColdReadAction {
    pub fn new(caster: String, target: String) -> Self {
        ColdReadAction { caster, target }
    }
}

impl ActiveTransition for ColdReadAction {
    fn simulate(&self, timeline: &AetTimeline) -> Vec<ProbableEvent> {
        vec![]
    }
    fn act(&self, timeline: &AetTimeline) -> ActivateResult {
        Ok(format!("coldread {}", self.target))
    }
}
