use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Hex {
    pub q: i32,
    pub r: i32,
}

impl Hex {
    pub fn neighbors(&self) -> Vec<Hex> {
        let deltas = [(1,0),(1,-1),(0,-1),(-1,0),(-1,1),(0,1)];
        deltas.iter().map(|(dq, dr)| Hex{ q: self.q + dq, r: self.r + dr }).collect()
    }

    pub fn is_adjacent(&self, hex: Hex) -> bool {
        self.neighbors().contains(&hex)
    }
}