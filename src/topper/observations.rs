use crate::timeline::TimeSlice;
use regex::{Captures, Match, Regex, RegexSet, RegexSetBuilder};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use std::time::{Duration, Instant};

pub trait EnumFromArgs {
    fn enum_from_args(observation_name: &str, arguments: Vec<String>) -> Self;
}

lazy_static! {
    static ref ANSI: Regex =
        Regex::new(r"(\x1b\[[\x30-\x3F]*[\x20-\x2F]*[\x40-\x7E]|\r\n)").unwrap();
}

pub fn strip_ansi(line: &String) -> String {
    ANSI.replace_all(line.as_ref(), "").into()
}

#[derive(Clone, Serialize, Deserialize, Debug)]
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
                    "You" | "you" | "yourself" | "your" | "Your" => slice.me.clone(),
                    x => x.to_string(),
                },
                None => "".to_string(),
            },
            ArgumentCapture::Literal(string) => string.clone(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
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
    fn try_get_arguments<'t, O, P>(
        &self,
        slice: &TimeSlice<O, P>,
        regex: &Regex,
        line: &String,
    ) -> Option<Vec<String>> {
        if let Some(captures) = regex.captures(line) {
            if self.args.len() == 0 {
                Some(vec![])
            } else {
                Some(
                    self.args
                        .iter()
                        .map(|arg| arg.get_argument(slice, &captures))
                        .collect(),
                )
            }
        } else {
            None
        }
    }
}

pub struct ObservationParser<O> {
    mappings: Vec<ObservationMapping>,
    pub regexes: Vec<Regex>,
    // regex_set: RegexSet,
    observation_creator: fn(&String, Vec<String>) -> O,
}

lazy_static! {
    pub static ref BENCHMARKS: Mutex<Vec<u128>> = Mutex::new(vec![]);
}

impl<O> ObservationParser<O>
where
    O: std::fmt::Debug,
{
    pub fn new(
        mappings: Vec<ObservationMapping>,
        observation_creator: fn(&String, Vec<String>) -> O,
    ) -> Self {
        let regexes: Vec<Regex> = mappings
            .iter()
            .map(|mapping| Regex::new(&mapping.regex.clone()).unwrap())
            .collect();
        // let regex_set = RegexSetBuilder::new(mappings.iter().map(|mapping| mapping.regex.clone()))
        //     .size_limit(1 << 24)
        //     .build()
        //     .unwrap();
        ObservationParser {
            regexes,
            // regex_set,
            mappings,
            observation_creator,
        }
    }

    pub fn write(&self, path: String) {
        use std::fs::File;
        use std::io::Write;
        let mut file = File::create(path).unwrap();
        writeln!(
            &mut file,
            "{}",
            serde_json::to_string_pretty(&self.mappings).unwrap_or("".to_string())
        );
    }

    pub fn observe<P>(&self, slice: &TimeSlice<O, P>) -> Vec<O> {
        let mut observations = Vec::new();
        {
            let mut benchmarks = BENCHMARKS.lock().unwrap();
            if benchmarks.len() != self.regexes.len() {
                benchmarks.fill(0);
                benchmarks.resize(self.regexes.len(), 0);
            }
        }
        for (line, idx) in slice.lines.iter() {
            let stripped = strip_ansi(line);
            // for match_num in self.regex_set.matches(&stripped) {
            //     let mapping = self.mappings.get(match_num).unwrap();
            //     let regex = self.regexes.get(match_num).unwrap();
            //     let arguments = mapping.get_arguments(&slice, &regex, &stripped);
            //     observations.push((self.observation_creator)(
            //         &mapping.observation_name,
            //         arguments,
            //     ));
            // }
            for (match_num, regex) in self.regexes.iter().enumerate() {
                let now = Instant::now();
                let mapping = self.mappings.get(match_num).unwrap();
                if let Some(arguments) = mapping.try_get_arguments(&slice, &regex, &stripped) {
                    observations.push((self.observation_creator)(
                        &mapping.observation_name,
                        arguments,
                    ));
                    log::info!("{:?}", observations.get(observations.len() - 1));
                }
                *BENCHMARKS.lock().unwrap().get_mut(match_num).unwrap() +=
                    Instant::now().duration_since(now).as_nanos();
            }
        }
        observations
    }
}
