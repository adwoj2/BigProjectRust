use crate::hexgrid::Hex;
use std::collections::{VecDeque, HashMap, HashSet};

pub fn movement_range(start: Hex, movement: i32, grid_boundary: Hex) -> HashMap<Hex, (i32, Vec<Hex>)> {
    if movement <= 0 {
        let mut map = HashMap::new();
        map.insert(start, (0, vec![start]));
        return map;
    }
    
    let mut visited: HashMap<Hex, (i32, Vec<Hex>)> = HashMap::new();
    let mut frontier = VecDeque::new();

    frontier.push_back((start, 0, vec![start]));

    while let Some((hex, dist, path)) = frontier.pop_front() {
        if visited.contains_key(&hex) { continue; }
        visited.insert(hex, (dist, path.clone()));

        if dist >= movement {
            continue;
        }

        for n in hex_neighbors(hex, grid_boundary) {
            if !visited.contains_key(&n) {
                let mut new_path = path.clone();
                new_path.push(n);
                frontier.push_back((n, dist + 1, new_path));
            }
        }
    }

    visited
}

/// Axial hex neighbors
pub fn hex_neighbors(hex: Hex, grid_boundary: Hex) -> Vec<Hex> {
    let directions_even = [
        (0, -1), (1, -1), (1, 0),
        (0, 1), (-1, 0), (-1, -1),
    ];
    
    let directions_odd = [
        (0, -1), (1, 0), (1, 1),
        (0, 1), (-1, 1), (-1, 0),
    ];

    let directions = if hex.q % 2 == 0 {
        &directions_even
    } else {
        &directions_odd
    };

    directions.iter()
        .map(|(dq, dr)| Hex { q: hex.q + dq, r: hex.r + dr })
        .filter(|h| h.q >= 0 && h.r >= 0 && h.q <= grid_boundary.q && h.r <= grid_boundary.r)
        .collect()
}

// pub fn bfs_path(start: Hex, goal: Hex, is_walkable: &dyn Fn(Hex) -> bool) -> Option<Vec<Hex>> {
//     let mut frontier = VecDeque::new();
//     frontier.push_back(start);
//     let mut came_from: HashMap<Hex, Option<Hex>> = HashMap::new();
//     came_from.insert(start, None);


//     while let Some(current) = frontier.pop_front() {
//         if current == goal { break; }
//         for neigh in current.neighbors() {
//             if !is_walkable(neigh) { continue; }
//             if !came_from.contains_key(&neigh) {
//                 frontier.push_back(neigh);
//                 came_from.insert(neigh, Some(current));
//             }
//         }
//     }


//     if !came_from.contains_key(&goal) { return None; }
//     let mut path = Vec::new();
//     let mut cur = Some(goal);
//     while let Some(pos) = cur {
//         path.push(pos);
//         cur = came_from[&pos];
//     }
//     path.reverse();
//     Some(path)
// }

pub fn bfs_path(start: Hex, goal: Hex, grid_boundary: Hex) -> Vec<Hex> {
    use std::collections::{HashMap, VecDeque};

    if start == goal {
        return vec![start];
    }

    let mut frontier = VecDeque::new();
    let mut came_from: HashMap<Hex, Hex> = HashMap::new();

    frontier.push_back(start);

    while let Some(current) = frontier.pop_front() {
        for neighbor in hex_neighbors(current, grid_boundary) {
            if !came_from.contains_key(&neighbor) && current != start {
                came_from.insert(neighbor, current);

                if neighbor == goal {
                    // reconstruct path
                    let mut path = vec![goal];
                    let mut cur = goal;
                    while cur != start {
                        cur = came_from[&cur];
                        path.push(cur);
                    }
                    path.reverse();
                    return path;
                }

                frontier.push_back(neighbor);
            }
        }
    }

    vec![]
}