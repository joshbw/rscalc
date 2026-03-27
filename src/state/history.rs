/// Calculation history: ordered list of (expression, result) pairs.

/// A single history entry.
#[derive(Debug, Clone)]
pub struct HistoryEntry {
    pub expression: String,
    pub result: String,
}

/// Calculator history.
#[derive(Debug, Default)]
pub struct History {
    entries: Vec<HistoryEntry>,
}

impl History {
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a new entry to the history.
    pub fn push(&mut self, expression: String, result: String) {
        self.entries.push(HistoryEntry { expression, result });
    }

    /// Get all history entries.
    pub fn entries(&self) -> &[HistoryEntry] {
        &self.entries
    }

    /// Clear all history.
    pub fn clear(&mut self) {
        self.entries.clear();
    }

    /// Get the number of entries.
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Check if history is empty.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}
