/*
pub struct TranspositionTable {
    entries: Vec<TranspositionTableEntry>,
    size: usize,
}

impl TranspositionTable {
    pub fn with_size(size: usize) -> Self {
        Self {
            entries: Vec::with_capacity(size)
        }
    }

    pub fn lookup(&self, hash: u64) -> Option<TranspositionTableEntry> {
        let index = hash % self.size;

    }
}

pub struct TranspositionTableEntry {
    hash: u64,
    action: Action,
    score: i16,
    depth: u8,
    alpha: bool,
    beta: bool,
}*/