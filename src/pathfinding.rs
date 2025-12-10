use crate::hexgrid::Hex;
use std::collections::{VecDeque, HashMap, HashSet};


pub fn bfs_path(start: Hex, goal: Hex, is_walkable: &dyn Fn(Hex) -> bool) -> Option<Vec<Hex>> {
    let mut frontier = VecDeque::new();
    frontier.push_back(start);
    let mut came_from: HashMap<Hex, Option<Hex>> = HashMap::new();
    came_from.insert(start, None);


    while let Some(current) = frontier.pop_front() {
        if current == goal { break; }
        for neigh in current.neighbors() {
            if !is_walkable(neigh) { continue; }
            if !came_from.contains_key(&neigh) {
                frontier.push_back(neigh);
                came_from.insert(neigh, Some(current));
            }
        }
    }


    if !came_from.contains_key(&goal) { return None; }
    let mut path = Vec::new();
    let mut cur = Some(goal);
    while let Some(pos) = cur {
        path.push(pos);
        cur = came_from[&pos];
    }
    path.reverse();
    Some(path)
}