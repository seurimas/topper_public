use crate::aetolia::timeline::{AetObservation, AetTimeSlice, AetTimeline, CombatAction};
use crate::timeline::CType;
use crate::topper::{DatabaseModule, TopperMessage, TopperModule, TopperRequest, TopperResponse};
use regex::Regex;
use std::collections::HashMap;

pub fn get_target_priority(db: &DatabaseModule, who: &String) -> Option<i32> {
    db.get_json::<i32>("target_priority", who)
}

fn set_target_priority(db: &DatabaseModule, who: &String, target_priority: i32) {
    db.insert_json::<i32>("target_priority", who, target_priority);
}

lazy_static! {
    static ref SLAIN_BY: Regex = Regex::new(r"^(\w+) has been slain by (.*)\.$").unwrap();
    static ref NO_SUCH_TARGET: Regex =
        Regex::new(r"^You can find no such target as '(\w+)'\.$").unwrap();
}

const AGGRO_DURATION: CType = 100 * 60 * 10;

#[derive(PartialEq, Clone, Copy)]
pub enum Tether {
    Spirit,
    Shadow,
    Neutral,
}

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
    fn highest_priority(&self, db: &DatabaseModule) -> Option<&(&String, &Aggro)>;
}

#[derive(Default)]
pub struct GroupModule {
    now: CType,
    my_tether: Option<Tether>,
    last_call: Option<(CType, String)>,
    aggro: HashMap<String, Aggro>,
}

impl GroupModule {
    pub fn new(db: &DatabaseModule) -> Self {
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
    fn call_target(&self, tether: Tether, in_room: bool, db: &DatabaseModule) -> Option<String> {
        self.aggro
            .iter()
            .filter(|(who, aggro)| {
                if self.now - aggro.last_seen > AGGRO_DURATION {
                    false
                } else if Some(tether) != aggro.tether {
                    false
                } else if in_room && !aggro.in_room {
                    false
                } else {
                    true
                }
            })
            .max_by_key(|(who, aggro)| get_target_priority(db, who).unwrap_or(0))
            .map(|(who, _aggro)| format!("X {}", who))
    }
    fn call_target_list(
        &self,
        tether: Tether,
        in_room: bool,
        db: &DatabaseModule,
    ) -> Option<String> {
        let mut sorted_targets = self
            .aggro
            .iter()
            .filter(|(who, aggro)| {
                if self.now - aggro.last_seen > AGGRO_DURATION {
                    false
                } else if Some(tether) != aggro.tether {
                    false
                } else if in_room && !aggro.in_room {
                    false
                } else {
                    true
                }
            })
            .collect::<Vec<(&String, &Aggro)>>();
        sorted_targets.sort_by_key(|(who, aggro)| get_target_priority(db, who).unwrap_or(0));
        let target_list = sorted_targets
            .into_iter()
            .map(|(who, aggro)| who.clone())
            .collect::<Vec<String>>()
            .join(", ");
        Some(format!("X{}", target_list))
    }
}

impl GroupData for Vec<(&String, &Aggro)> {
    fn highest_priority(&self, db: &DatabaseModule) -> Option<&(&String, &Aggro)> {
        self.iter()
            .max_by_key(|(who, aggro)| get_target_priority(db, who).unwrap_or(0))
    }
}

impl<'s> TopperModule<'s> for GroupModule {
    type Siblings = (&'s String, &'s AetTimeline, &'s DatabaseModule);
    fn handle_message(
        &mut self,
        message: &TopperMessage,
        (me, timeline, db): Self::Siblings,
    ) -> Result<TopperResponse, String> {
        let mut calls = None;
        match message {
            TopperMessage::AetEvent(timeslice) => {
                self.now = timeslice.time;
                if let Some(observations) = &timeslice.observations {
                    for event in observations.iter() {
                        match event {
                            AetObservation::CombatAction(CombatAction {
                                caster, target, ..
                            }) => {
                                {
                                    let caster = self.aggro.entry(caster.to_string()).or_default();
                                    caster.in_room = true;
                                }
                                {
                                    let target = self.aggro.entry(target.to_string()).or_default();
                                    target.in_room = true;
                                    target.last_hit = self.now;
                                }
                            }
                            _ => {}
                        }
                    }
                }
                for line in timeslice.lines.iter() {
                    if let Some(captures) = SLAIN_BY.captures(&line.0) {
                        {
                            let target = self
                                .aggro
                                .entry(captures.get(1).unwrap().as_str().to_string())
                                .or_default();
                            target.in_room = false;
                        }
                    } else if let Some(captures) = NO_SUCH_TARGET.captures(&line.0) {
                        {
                            let target = self
                                .aggro
                                .entry(captures.get(1).unwrap().as_str().to_string())
                                .or_default();
                            target.in_room = false;
                        }
                    }
                }
            }
            TopperMessage::Request(TopperRequest::Group(command)) => match command.as_ref() {
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
                _ => {
                    println!("No such command: {}", command);
                }
            },
            _ => {}
        }
        if let Some(calls) = calls {
            Ok(TopperResponse::passive("group".to_string(), calls))
        } else {
            Ok(TopperResponse::silent())
        }
    }
}
