use crate::aetolia::timeline::*;
use crate::timeline::types::*;
use regex::{Captures, Match, Regex, RegexSet};
use serde::{Deserialize, Serialize};

#[cfg(test)]
mod observer_tests {
    use super::*;

    #[test]
    fn test_combat_action_target() {
        let slice = AetTimeSlice {
            observations: None,
            lines: vec![(
                "You use Assassination Doublestab on Benedicto.".to_string(),
                0,
            )],
            prompt: AetPrompt::Promptless,
            time: 0,
            me: "Seurimas".into(),
        };
        let observer: ObservationParser<AetObservation> = Default::default();
        let observed = observer.observe(slice);
        let mut expected = Vec::new();
        expected.push(CombatAction::observation(
            "Seurimas",
            "Benedicto",
            "Assassination",
            "Doublestab",
            "",
        ));
        assert_eq!(observed, expected);
    }

    #[test]
    fn test_combat_action_target_annotated() {
        let slice = AetTimeSlice {
            observations: None,
            lines: vec![(
                "You use Assassination Bite (scytherus) on Benedicto.".to_string(),
                0,
            )],
            prompt: AetPrompt::Promptless,
            time: 0,
            me: "Seurimas".into(),
        };
        let observer: ObservationParser<AetObservation> = Default::default();
        let observed = observer.observe(slice);
        let mut expected = Vec::new();
        expected.push(CombatAction::observation(
            "Seurimas",
            "Benedicto",
            "Assassination",
            "Bite",
            "scytherus",
        ));
        assert_eq!(observed, expected);
    }

    #[test]
    fn test_combat_action_no_target() {
        let slice = AetTimeSlice {
            observations: None,
            lines: vec![("You use Assassination Warding.".to_string(), 0)],
            prompt: AetPrompt::Promptless,
            time: 0,
            me: "Seurimas".into(),
        };
        let observer: ObservationParser<AetObservation> = Default::default();
        let observed = observer.observe(slice);
        let mut expected = Vec::new();
        expected.push(CombatAction::observation(
            "Seurimas",
            "",
            "Assassination",
            "Warding",
            "",
        ));
        assert_eq!(observed, expected);
    }

    #[test]
    fn test_touch_tree() {
        let slice = AetTimeSlice {
            observations: None,
            lines: vec![
                ("You touch the tree of life tattoo.".to_string(), 0),
                ("Benedicto touches a tree of life tattoo.".to_string(), 1),
            ],
            prompt: AetPrompt::Promptless,
            time: 0,
            me: "Seurimas".into(),
        };
        let observer: ObservationParser<AetObservation> = Default::default();
        let observed = observer.observe(slice);
        let mut expected = Vec::new();
        expected.push(CombatAction::observation(
            "Seurimas", "", "Tattoos", "Tree", "",
        ));
        expected.push(CombatAction::observation(
            "Benedicto",
            "",
            "Tattoos",
            "Tree",
            "",
        ));
        assert_eq!(observed, expected);
    }
}

lazy_static! {
    static ref ANSI: Regex =
        Regex::new(r"(\x1b\[[\x30-\x3F]*[\x20-\x2F]*[\x40-\x7E]|\r\n)").unwrap();
}

pub fn strip_ansi(line: &String) -> String {
    ANSI.replace_all(line.as_ref(), "").into()
}

#[derive(Clone, Serialize, Debug)]
enum ArgumentCapture {
    Group(usize),
    GroupAsTarget(usize),
    Literal(String),
}

impl ArgumentCapture {
    fn get_argument<'t, O, P>(&self, slice: &TimeSlice<O, P>, captures: &Captures<'t>) -> String {
        match self {
            ArgumentCapture::Group(idx) => match captures.get(*idx) {
                Some(text) => text.as_str().to_string(),
                None => "".to_string(),
            },
            ArgumentCapture::GroupAsTarget(idx) => match captures.get(*idx) {
                Some(text) => match text.as_str() {
                    "You" | "you" | "yourself" | "your" => slice.me.clone(),
                    x => x.to_string(),
                },
                None => "".to_string(),
            },
            ArgumentCapture::Literal(string) => string.clone(),
        }
    }
}

#[derive(Clone, Serialize, Debug)]
pub struct ObservationMapping {
    regex: String,
    args: Vec<ArgumentCapture>,
    observation_name: String,
}

impl ObservationMapping {
    fn get_arguments<'t, O, P>(
        &self,
        slice: &TimeSlice<O, P>,
        regex: &Regex,
        line: &String,
    ) -> Vec<String> {
        if self.args.len() == 0 {
            vec![]
        } else {
            let captures = regex.captures(line).unwrap();
            self.args
                .iter()
                .map(|arg| arg.get_argument(slice, &captures))
                .collect()
        }
    }
}

lazy_static! {
    static ref COMBAT_ACTION_MAPPING: ObservationMapping = ObservationMapping {
        regex: r"^(\w+) uses? (\w+) (\w+ ?\w*)( \((.*)\))?( on (.*))?.$".to_string(),
        args: vec![
            ArgumentCapture::GroupAsTarget(1),
            ArgumentCapture::GroupAsTarget(7),
            ArgumentCapture::Group(2),
            ArgumentCapture::Group(3),
            ArgumentCapture::Group(5)
        ],
        observation_name: "CombatAction".to_string(),
    };
    static ref TREE_MAPPING: ObservationMapping = ObservationMapping {
        regex: r"^(\w+) touch(es)? (a|the) tree of life tattoo.$".to_string(),
        args: vec![
            ArgumentCapture::GroupAsTarget(1),
            ArgumentCapture::Literal("".to_string()),
            ArgumentCapture::Literal("Tattoos".to_string()),
            ArgumentCapture::Literal("Tree".to_string()),
            ArgumentCapture::Literal("".to_string()),
        ],
        observation_name: "CombatAction".to_string(),
    };
}

pub struct ObservationParser<O> {
    mappings: Vec<ObservationMapping>,
    regexes: Vec<Regex>,
    regex_set: RegexSet,
    observation_creator: fn(&String, Vec<String>) -> O,
}

fn aet_observation_creator(observation_name: &String, arguments: Vec<String>) -> AetObservation {
    match observation_name.as_ref() {
        "CombatAction" => CombatAction::observation(
            &arguments.get(0).unwrap(),
            &arguments.get(1).unwrap(),
            &arguments.get(2).unwrap(),
            &arguments.get(3).unwrap(),
            &arguments.get(4).unwrap(),
        ),
        _ => AetObservation::Diverts,
    }
}

impl Default for ObservationParser<AetObservation> {
    fn default() -> ObservationParser<AetObservation> {
        ObservationParser::new(
            vec![COMBAT_ACTION_MAPPING.clone(), TREE_MAPPING.clone()],
            aet_observation_creator,
        )
    }
}

impl<O> ObservationParser<O> {
    pub fn new(
        mappings: Vec<ObservationMapping>,
        observation_creator: fn(&String, Vec<String>) -> O,
    ) -> Self {
        let regexes = mappings
            .iter()
            .map(|mapping| Regex::new(&mapping.regex.clone()).unwrap())
            .collect();
        let regex_set =
            RegexSet::new(mappings.iter().map(|mapping| mapping.regex.clone())).unwrap();
        ObservationParser {
            regexes,
            regex_set,
            mappings,
            observation_creator,
        }
    }

    pub fn observe<P>(&self, slice: TimeSlice<O, P>) -> Vec<O> {
        let mut observations = Vec::new();
        for (line, idx) in slice.lines.iter() {
            let stripped = strip_ansi(line);
            for match_num in self.regex_set.matches(&stripped) {
                let mapping = self.mappings.get(match_num).unwrap();
                let regex = self.regexes.get(match_num).unwrap();
                let arguments = mapping.get_arguments(&slice, &regex, &stripped);
                observations.push((self.observation_creator)(
                    &mapping.observation_name,
                    arguments,
                ));
            }
        }
        observations
    }
}
