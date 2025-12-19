use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Hex {
    pub q: i32,
    pub r: i32,
}

impl Hex {
    pub fn neighbors(&self, grid_width: i32, grid_height: i32) -> Vec<Hex> {
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
            .filter(|h| h.q >= 0 && h.r >= 0 && h.q <= grid_width && h.r <= grid_height)
            .collect()
    }

    pub fn is_adjacent(&self, hex: Hex, grid_width: i32, grid_height: i32) -> bool {
        self.neighbors(grid_width, grid_height).contains(&hex)
    }

    pub fn range_to_area(&self, range: i32, grid_width: i32, grid_height: i32) -> Vec<Hex> {
        let mut area = Vec::new();

        let mut visited = std::collections::HashSet::new();
        let mut frontier = vec![*self];
        visited.insert((self.q, self.r));

        for _ in 0..=range {
            let mut next_frontier = Vec::new();
            for hex in frontier {
                area.push(hex);

                const DIRECTIONS_EVEN: [(i32, i32); 6] =
                    [(0, -1), (1, -1), (1, 0), (0, 1), (-1, 0), (-1, -1)];
                const DIRECTIONS_ODD: [(i32, i32); 6] =
                    [(0, -1), (1, 0), (1, 1), (0, 1), (-1, 1), (-1, 0)];
                let directions = if hex.q % 2 == 0 {
                    &DIRECTIONS_EVEN
                } else {
                    &DIRECTIONS_ODD
                };

                for (dq, dr) in directions {
                    let neighbor = Hex {
                        q: hex.q + dq,
                        r: hex.r + dr,
                    };
                    if neighbor.q >= 0
                        && neighbor.q < grid_width
                        && neighbor.r >= 0
                        && neighbor.r < grid_height
                    {
                        if !visited.contains(&(neighbor.q, neighbor.r)) {
                            visited.insert((neighbor.q, neighbor.r));
                            next_frontier.push(neighbor);
                        }
                    }
                }
            }
            frontier = next_frontier;
        }

        area
    }
}
