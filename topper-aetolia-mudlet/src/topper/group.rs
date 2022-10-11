use regex::Regex;
use std::collections::HashMap;
use topper_aetolia::timeline::{
    for_agent, AetObservation, AetTimeSlice, AetTimeline, CombatAction,
};
use topper_core::observations::strip_ansi;
use topper_core::timeline::db::DatabaseModule;
use topper_core::timeline::CType;
use topper_core_mudlet::topper::{TopperMessage, TopperModule, TopperRequest, TopperResponse};

use super::battle_stats::BattleStats;
use super::db::AetMudletDatabaseModule;

pub fn get_target_priority(db: &AetMudletDatabaseModule, who: &String) -> Option<i32> {
    db.get_json::<i32>("target_priority", who)
}

fn set_target_priority(db: &AetMudletDatabaseModule, who: &String, target_priority: i32) {
    db.insert_json::<i32>("target_priority", who, target_priority);
}

fn capitalize(s: String) -> String {
    format!(
        "{}{}",
        (&s[..1].to_string()).to_uppercase(),
        &s[1..].to_lowercase()
    )
}

lazy_static! {
    static ref PLAYER: Regex = Regex::new(r"^\w+$").unwrap();
    static ref SLAIN_BY: Regex = Regex::new(r"^(\w+) has been slain by (.*)\.$").unwrap();
    static ref NO_SUCH_TARGET: Regex =
        Regex::new(r"^You can find no such target as '(\w+)'\.$").unwrap();
    static ref WHO_LINE: Regex = Regex::new(r"^\s+(\w+) - .{38} -(| v\d+)").unwrap();
    static ref PRIORITY: Regex = Regex::new(r"^priority (\w+) (\d+)$").unwrap();
}

const AGGRO_DURATION: CType = 100 * 60 * 10;

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Tether {
    Spirit,
    Shadow,
    Neutral,
}

#[derive(Debug)]
pub struct Aggro {
    tether: Option<Tether>,
    last_seen: CType,
    in_room: bool,
    last_hit: CType,
}

impl Default for Aggro {
    fn default() -> Self {
        Aggro {
            tether: None,
            last_seen: 0,
            last_hit: 0,
            in_room: false,
        }
    }
}

impl Aggro {
    fn new_by_city(city: &String) -> Self {
        Aggro {
            tether: match city.as_ref() {
                "Spinesreach" | "Bloodloch" => Some(Tether::Shadow),
                "Enorian" | "Duiran" => Some(Tether::Spirit),
                "(none)" => Some(Tether::Neutral),
                _ => None,
            },
            last_seen: 0,
            last_hit: 0,
            in_room: false,
        }
    }
}

trait GroupData {
    fn highest_priority(&self, db: &AetMudletDatabaseModule) -> Option<&(&String, &Aggro)>;
}

#[derive(Default)]
pub struct GroupModule {
    now: CType,
    my_tether: Option<Tether>,
    last_call: Option<(CType, String)>,
    aggro: HashMap<String, Aggro>,
}

impl GroupModule {
    pub fn new(db: &AetMudletDatabaseModule) -> Self {
        let mut aggro = HashMap::new();
        for character in db.get_characters() {
            aggro.insert(character.name, Aggro::new_by_city(&character.city));
        }
        GroupModule {
            aggro,
            last_call: None,
            my_tether: None,
            now: 0,
        }
    }
    fn get_active_for(&self, in_last: CType, tether: Tether) -> Vec<(&String, &Aggro)> {
        self.aggro
            .iter()
            .filter(|(who, aggro)| {
                self.now - aggro.last_seen < in_last && Some(tether) == aggro.tether
            })
            .collect::<Vec<(&String, &Aggro)>>()
    }
    fn get_filter(&self, tether: Tether, in_room: bool) -> impl FnMut(&(&String, &Aggro)) -> bool {
        let now = self.now;
        move |(who, aggro)| {
            if now - aggro.last_seen > AGGRO_DURATION {
                false
            } else if aggro.last_seen == 0 {
                false
            } else if Some(tether) != aggro.tether {
                false
            } else if in_room && !aggro.in_room {
                false
            } else {
                true
            }
        }
    }
    fn call_target(
        &self,
        tether: Tether,
        in_room: bool,
        db: &AetMudletDatabaseModule,
    ) -> Option<String> {
        self.aggro
            .iter()
            .filter(self.get_filter(tether, in_room))
            .filter(|(who, aggro)| get_target_priority(db, who).is_some())
            .min_by_key(|(who, aggro)| get_target_priority(db, who).unwrap_or(0))
            .map(|(who, _aggro)| format!("X {}", who))
    }
    fn call_target_list(
        &self,
        tether: Tether,
        in_room: bool,
        db: &AetMudletDatabaseModule,
    ) -> Option<String> {
        let mut sorted_targets = self
            .aggro
            .iter()
            .filter(self.get_filter(tether, in_room))
            .filter(|(who, aggro)| get_target_priority(db, who).is_some())
            .collect::<Vec<(&String, &Aggro)>>();
        sorted_targets.sort_by_key(|(who, aggro)| get_target_priority(db, who).unwrap());
        let target_list = sorted_targets
            .into_iter()
            .map(|(who, aggro)| who.clone())
            .collect::<Vec<String>>()
            .join(", ");
        Some(format!(
            "X {}%%wt Target list: {}",
            target_list, target_list
        ))
    }
}

impl GroupData for Vec<(&String, &Aggro)> {
    fn highest_priority(&self, db: &AetMudletDatabaseModule) -> Option<&(&String, &Aggro)> {
        self.iter()
            .max_by_key(|(who, aggro)| get_target_priority(db, who).unwrap_or(0))
    }
}

impl<'s> TopperModule<'s, AetTimeSlice, BattleStats> for GroupModule {
    type Siblings = (&'s String, &'s mut AetTimeline, &'s AetMudletDatabaseModule);
    fn handle_message(
        &mut self,
        message: &TopperMessage<AetTimeSlice>,
        (me, mut timeline, db): Self::Siblings,
    ) -> Result<TopperResponse<BattleStats>, String> {
        let mut calls = None;
        match message {
            TopperMessage::TimeSlice(timeslice) => {
                self.now = timeslice.time;
                if let Some(observations) = &timeslice.observations {
                    for event in observations.iter() {
                        match event {
                            AetObservation::CombatAction(CombatAction {
                                caster, target, ..
                            }) => {
                                if PLAYER.is_match(caster) {
                                    let caster_aggro =
                                        self.aggro.entry(caster.to_string()).or_default();
                                    caster_aggro.last_seen = self.now;
                                    caster_aggro.in_room = true;
                                    if get_target_priority(db, &caster.to_string())
                                        .unwrap_or_default()
                                        == 0
                                    {
                                        println!("Adding default priority for {}", caster);
                                        set_target_priority(db, &caster.to_string(), 50);
                                    }
                                }
                                if PLAYER.is_match(target) {
                                    let target_aggro =
                                        self.aggro.entry(target.to_string()).or_default();
                                    target_aggro.last_seen = self.now;
                                    target_aggro.in_room = true;
                                    target_aggro.last_hit = self.now;
                                    if get_target_priority(db, &target.to_string())
                                        .unwrap_or_default()
                                        == 0
                                    {
                                        println!("Adding default priority for {}", target);
                                        set_target_priority(db, &target.to_string(), 50);
                                    }
                                }
                            }
                            _ => {}
                        }
                    }
                }
                for line in timeslice.lines.iter() {
                    let line = strip_ansi(&line.0);
                    if let Some(captures) = SLAIN_BY.captures(&line) {
                        let target = self
                            .aggro
                            .entry(captures.get(1).unwrap().as_str().to_string())
                            .or_default();
                        target.in_room = false;
                    } else if let Some(captures) = NO_SUCH_TARGET.captures(&line) {
                        let target_name = capitalize(captures.get(1).unwrap().as_str().to_string());
                        let target = self.aggro.entry(target_name.clone()).or_default();
                        target.in_room = false;
                        for_agent(&mut timeline.state, &target_name, &|me| {
                            me.room_id = 0;
                        });
                    } else if let Some(captures) = WHO_LINE.captures(&line) {
                        let target = self
                            .aggro
                            .entry(captures.get(1).unwrap().as_str().to_string())
                            .or_default();
                        target.last_seen = self.now;
                    }
                }
            }
            TopperMessage::Request(TopperRequest::ModuleMsg(module, command)) => {
                if module.eq("group") {
                    match command.as_ref() {
                        "call_list" => {
                            if let Some(tether) = self.my_tether {
                                calls = self.call_target_list(tether, false, db);
                            } else {
                                println!("No tether. Set one before calling.");
                            }
                        }
                        "call_in" => {
                            if let Some(tether) = self.my_tether {
                                calls = self.call_target(tether, true, db);
                            } else {
                                println!("No tether. Set one before calling.");
                            }
                        }
                        "call_all" => {
                            if let Some(tether) = self.my_tether {
                                calls = self.call_target(tether, false, db);
                            } else {
                                println!("No tether. Set one before calling.");
                            }
                        }
                        "reset" => {
                            self.my_tether = None;
                            self.aggro = HashMap::new();
                        }
                        "tether shadow" => {
                            self.my_tether = Some(Tether::Spirit);
                        }
                        "tether spirit" => {
                            self.my_tether = Some(Tether::Shadow);
                        }
                        "check" => {
                            println!("Aggros: {:?}", self.aggro);
                        }
                        _ => {
                            if let Some(captures) = PRIORITY.captures(command) {
                                let who = captures.get(1).unwrap().as_str().to_string();
                                let priority = captures
                                    .get(2)
                                    .unwrap()
                                    .as_str()
                                    .to_string()
                                    .parse::<i32>()
                                    .unwrap();
                                set_target_priority(db, &who, priority);
                                println!("Set {} to {}", who, priority);
                            } else {
                                println!("No such command: {}", command);
                            }
                        }
                    }
                }
            }
            _ => {}
        }
        if let Some(calls) = calls {
            Ok(TopperResponse::passive("group".to_string(), calls))
        } else {
            Ok(TopperResponse::silent())
        }
    }
}
