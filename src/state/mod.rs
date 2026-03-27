/// Calculator state management: unifies settings, memory, history, and result.
pub mod history;
pub mod memory;
pub mod settings;

use calc_manager::prelude::Rational;
use calc_manager::ratpack::constants::RatpackConstants;

use self::history::History;
use self::memory::Memory;
use self::settings::Settings;

/// The complete calculator state.
pub struct CalcState {
    pub settings: Settings,
    pub memory: Memory,
    pub history: History,
    pub last_result: Option<Rational>,
    /// Cached constants keyed by (radix, precision) to avoid recomputing
    /// transcendentals (π, e, ln2, ln10) on every evaluation.
    constants_cache: Option<(u32, i32, RatpackConstants)>,
}

impl CalcState {
    pub fn new() -> Self {
        Self {
            settings: Settings::default(),
            memory: Memory::new(),
            history: History::new(),
            last_result: None,
            constants_cache: None,
        }
    }

    /// Ensure constants are computed and cached for the given radix/precision.
    /// Call this before borrowing constants immutably.
    pub fn ensure_constants(&mut self, radix: u32, precision: i32) {
        let needs_recompute = match &self.constants_cache {
            Some((r, p, _)) => *r != radix || *p != precision,
            None => true,
        };
        if needs_recompute {
            self.constants_cache =
                Some((radix, precision, RatpackConstants::new(radix, precision)));
        }
    }

    /// Get a reference to the cached constants. Panics if not yet computed.
    pub fn cached_constants(&self) -> &RatpackConstants {
        &self
            .constants_cache
            .as_ref()
            .expect("constants not initialized")
            .2
    }

    /// Eagerly warm the constants cache for the current settings.
    /// Call this during startup so the first evaluation doesn't pay the cost.
    pub fn warm_constants(&mut self) {
        let radix = self.settings.radix_type.to_radix();
        let precision = self.settings.precision();
        self.ensure_constants(radix, precision);
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
