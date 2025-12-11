use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Hex {
    pub q: i32,
    pub r: i32,
}

impl Hex {
    pub fn neighbors(&self, grid_boundary: Hex) -> Vec<Hex> {
        // directions for even-q and odd-q
        const DIRECTIONS_EVEN: [(i32, i32); 6] =
            [(0, -1), (1, -1), (1, 0), (0, 1), (-1, 0), (-1, -1)];

        const DIRECTIONS_ODD: [(i32, i32); 6] = [(0, -1), (1, 0), (1, 1), (0, 1), (-1, 1), (-1, 0)];

        let directions = if self.q % 2 == 0 {
            &DIRECTIONS_EVEN
        } else {
            &DIRECTIONS_ODD
        };

        directions
            .iter()
            .map(|(dq, dr)| Hex {
                q: self.q + dq,
                r: self.r + dr,
            })
            .filter(|h| h.q >= 0 && h.r >= 0 && h.q <= grid_boundary.q && h.r <= grid_boundary.r)
            .collect()
    }

    pub fn is_adjacent(&self, hex: Hex, grid_boundary: Hex) -> bool {
        self.neighbors(grid_boundary).contains(&hex)
    }
}
