/// Memory operations: MS, MR, MC, M+, M-.

use calc_manager::prelude::Rational;
use calc_manager::ratpack::arithmetic::{add_rat, sub_rat};

/// Calculator memory — a single value slot (following calc.exe design).
#[derive(Debug, Default)]
pub struct Memory {
    value: Option<Rational>,
}

impl Memory {
    pub fn new() -> Self {
        Self::default()
    }

    /// Store a value in memory (MS).
    pub fn store(&mut self, val: &Rational) {
        self.value = Some(val.dup());
    }

    /// Recall the memory value (MR). Returns None if empty.
    pub fn recall(&self) -> Option<&Rational> {
        self.value.as_ref()
    }

    /// Clear memory (MC).
    pub fn clear(&mut self) {
        self.value = None;
    }

    /// Add to memory (M+).
    pub fn add(&mut self, val: &Rational, precision: i32) {
        match &self.value {
            Some(existing) => {
                self.value = Some(add_rat(existing, val, precision));
            }
            None => {
                self.value = Some(val.dup());
            }
        }
    }

    /// Subtract from memory (M-).
    pub fn subtract(&mut self, val: &Rational, precision: i32) {
        match &self.value {
            Some(existing) => {
                self.value = Some(sub_rat(existing, val, precision));
            }
            None => {
                self.value = Some(val.negate());
            }
        }
    }

    /// Check if memory has a stored value.
    pub fn has_value(&self) -> bool {
        self.value.is_some()
    }
}
