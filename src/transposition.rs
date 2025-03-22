use shakmaty::Move;
use crate::score::Score;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum NodeType {
    Exact,
    UpperBound,
    LowerBound,
}

#[derive(Clone)]
pub struct TranspositionEntry {
    pub hash: u64,
    pub depth: i16,
    pub score: Score,
    pub node_type: NodeType,
    pub best_move: Option<Move>,
}

pub struct TranspositionTable {
    entries: Vec<Option<TranspositionEntry>>,
    size: usize,
}

impl TranspositionTable {
    pub fn new(size: usize) -> Self {
        Self {
            entries: vec![None; size],
            size,
        }
    }
    pub fn get(&self, hash: u64) -> Option<&TranspositionEntry> {
        let index = hash as usize % self.size;
        self.entries[index].as_ref().filter(|entry| entry.hash == hash)
    }
    pub fn insert(&mut self, hash: u64, depth: i16, score: Score, node_type: NodeType, best_move: Option<Move>) {
        let index = hash as usize % self.size;
        self.entries[index] = Some(TranspositionEntry { hash, depth, score, node_type, best_move });
    }
}
