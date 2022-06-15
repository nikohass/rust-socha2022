use game_sdk::action::Action;

pub const TT_SIZE: usize = 2_usize.pow(23);

#[derive(Clone, Copy)]
pub struct TranspositionTableEntry {
    pub value: i16,
    pub action: Action,
    pub depth: u8,
    pub hash: usize,
    pub alpha: bool,
    pub beta: bool,
}

impl TranspositionTableEntry {
    pub fn is_valid(&self) -> bool {
        self.depth != std::u8::MAX
    }
}

impl Default for TranspositionTableEntry {
    fn default() -> Self {
        Self {
            value: 0,
            action: Action::none(),
            depth: std::u8::MAX,
            hash: 0,
            alpha: false,
            beta: false,
        }
    }
}

pub struct TranspositionTable {
    entries: Vec<TranspositionTableEntry>,
}

impl TranspositionTable {
    pub fn insert(&mut self, hash: usize, new_entry: TranspositionTableEntry) {
        let index = hash % TT_SIZE;
        let entry = self.entries[index];
        let is_valid = entry.is_valid();
        if entry.depth <= new_entry.depth || !is_valid {
            self.entries[index] = new_entry;
        }
    }

    pub fn lookup(&self, hash: usize) -> Option<TranspositionTableEntry> {
        let entry = self.entries[hash % TT_SIZE];
        if entry.is_valid() && entry.hash == hash {
            Some(entry)
        } else {
            None
        }
    }
}

impl Default for TranspositionTable {
    fn default() -> Self {
        let entries = vec![TranspositionTableEntry::default(); TT_SIZE];
        Self { entries }
    }
}
