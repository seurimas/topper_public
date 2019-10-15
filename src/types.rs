use num_enum::TryFromPrimitive;
pub type CType = i32;

pub const BALANCE_SCALE: f32 = 100.0;

// Balances
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, TryFromPrimitive)]
#[repr(usize)]
pub enum BType {
    Balance,
    Equil,
    Elixir,

    SIZE,
}

// Stats
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, TryFromPrimitive)]
#[repr(u8)]
pub enum SType {
    Health,
    Mana,

    SIZE,
}

// Flags
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, TryFromPrimitive)]
#[repr(u16)]
pub enum FType {
    Dead,
    Shield,

    SIZE,
}

pub type StateChange = Box<Fn(&mut AgentState, &mut AgentState)>;

pub type StateMatcher = Box<Fn(&AgentState, &AgentState) -> bool>;

#[derive(Debug, Clone, Default)]
pub struct AgentState {
    pub balances: [CType; BType::SIZE as usize],
    pub stats: [CType; SType::SIZE as usize],
    pub max_stats: [CType; SType::SIZE as usize],
    pub flags: [bool; FType::SIZE as usize],
}

impl AgentState {
    pub fn wait(&mut self, duration: i32) {
        for i in 0..self.balances.len() {
            self.balances[i] -= duration;
        }
    }
}

pub fn target(matcher: StateMatcher) -> StateMatcher {
    Box::new(move |_me, them| matcher(them, _me))
}

pub fn alive() -> StateMatcher {
    Box::new(|me, _them| !me.flags[FType::Dead as usize])
}

pub fn has(balance: BType) -> StateMatcher {
    Box::new(move |me, _them| me.balances[balance as usize] <= 0)
}
