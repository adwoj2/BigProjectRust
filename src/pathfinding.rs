use crate::battlestate::BattleState;
use crate::hexgrid::Hex;
use std::collections::{HashMap, HashSet, VecDeque};

/// Return a map from reachable Hex -> (cost, path) using a breadth-first expansion limited by `movement`.
/// Paths include the start as the first element and the target as the last.
pub fn movement_range(
    start: Hex,
    movement: i32,
    grid_boundary: Hex,
    battle: &BattleState,
) -> HashMap<Hex, (i32, Vec<Hex>)> {
    if movement <= 0 {
        let mut map = HashMap::new();
        map.insert(start, (0, vec![start]));
        return map;
    }

    let mut visited: HashMap<Hex, (i32, Vec<Hex>)> = HashMap::new();
    let mut frontier: VecDeque<(Hex, i32, Vec<Hex>)> = VecDeque::new();

    frontier.push_back((start, 0, vec![start]));

    while let Some((hex, dist, path)) = frontier.pop_front() {
        if visited.contains_key(&hex) || !battle.is_passable_for_unit(start, hex) {
            continue;
        }

        visited.insert(hex, (dist, path.clone()));

        if dist >= movement {
            continue;
        }

        for neighbor in hex_neighbors(hex, grid_boundary) {
            if !visited.contains_key(&neighbor) {
                let mut new_path = path.clone();
                new_path.push(neighbor);
                frontier.push_back((neighbor, dist + 1, new_path));
            }
        }
    }

    visited
}

pub fn hex_neighbors(hex: Hex, grid_boundary: Hex) -> Vec<Hex> {
    const DIRECTIONS_EVEN: [(i32, i32); 6] = [(0, -1), (1, -1), (1, 0), (0, 1), (-1, 0), (-1, -1)];

    const DIRECTIONS_ODD: [(i32, i32); 6] = [(0, -1), (1, 0), (1, 1), (0, 1), (-1, 1), (-1, 0)];

    let directions = if hex.q % 2 == 0 {
        &DIRECTIONS_EVEN
    } else {
        &DIRECTIONS_ODD
    };

    directions
        .iter()
        .map(|(dq, dr)| Hex {
            q: hex.q + dq,
            r: hex.r + dr,
        })
        .filter(|h| h.q >= 0 && h.r >= 0 && h.q <= grid_boundary.q && h.r <= grid_boundary.r)
        .collect()
}

pub fn bfs_path(start: Hex, goal: Hex, grid_boundary: Hex, battle: &BattleState) -> Vec<Hex> {
    use std::collections::{HashMap, VecDeque};

    if start == goal {
        return vec![start];
    }

    let mut frontier: VecDeque<Hex> = VecDeque::new();
    let mut came_from: HashMap<Hex, Hex> = HashMap::new();
    let mut visited: HashSet<Hex> = HashSet::new();

    frontier.push_back(start);
    visited.insert(start);

    while let Some(current) = frontier.pop_front() {
        for neighbor in hex_neighbors(current, grid_boundary) {
            if visited.contains(&neighbor) || !battle.is_passable_for_unit(goal, neighbor) {
                continue;
            }

            came_from.insert(neighbor, current);
            visited.insert(neighbor);
            frontier.push_back(neighbor);

            if neighbor == goal {
                let mut path = Vec::new();
                let mut cur = goal;
                path.push(cur);
                while cur != start {
                    cur = came_from[&cur];
                    path.push(cur);
                }
                path.reverse();
                return path;
            }
        }
    }

    vec![]
}
