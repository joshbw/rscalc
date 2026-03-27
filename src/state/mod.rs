/// Calculator state management: unifies settings, memory, history, and result.

pub mod history;
pub mod memory;
pub mod settings;

use calc_manager::prelude::Rational;

use self::history::History;
use self::memory::Memory;
use self::settings::Settings;

/// The complete calculator state.
pub struct CalcState {
    pub settings: Settings,
    pub memory: Memory,
    pub history: History,
    pub last_result: Option<Rational>,
}

impl CalcState {
    pub fn new() -> Self {
        Self {
            settings: Settings::default(),
            memory: Memory::new(),
            history: History::new(),
            last_result: None,
        }
    }

    /// Clear all calculator state (settings are preserved).
    pub fn clear(&mut self) {
        self.memory.clear();
        self.history.clear();
        self.last_result = None;
    }
}

impl Default for CalcState {
    fn default() -> Self {
        Self::new()
    }
}
