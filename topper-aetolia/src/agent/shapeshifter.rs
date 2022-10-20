use super::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Howl {
    Terrorizing,
    Traumatic,
    Piercing,
    Paralyzing,
    Baleful,
    Rousing,
    Distasteful,
    Forceful,
    MindNumbing,
    StomachTurning,
    Claustrophobic,
    Screeching,
    Comforting,
    Rejuvenating,
    Ringing,
    Deep,
    Dumbing,
    Blurring,
    Disruptive,
    Serenading,
    Debilitating,
    Berserking,
    Angry,
    Wailing,
    Disturbing,
    Soothing,
    Invigorating,
    Enfeebling,
    Befuddling,
    Lulling,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct HowlingState {
    pub snarling: bool,
    pub echoing: bool,
    pub boneshaking: bool,
    pub attuning: bool,
    pub howls: [Option<Howl>; 3],
    pub time_since: CType,
}
