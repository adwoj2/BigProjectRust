use crate::character::Enemy;
use crate::character::Hero;
use crate::battlefield::Battlefield;


pub fn choose_attack_target(attacker: &Enemy, heroes: &[Hero], bf: &Battlefield) -> Option<usize> {
    // prosty algorytm: wybierz bohatera z najniższymi obecnymi hp albo tego, który może zostać powalony
    let mut best: Option<usize> = None;
    for (i, h) in heroes.iter().enumerate() {
        if h.stats.hp <= 0 { continue; }
        if best.is_none() || h.stats.hp < heroes[best.unwrap()].stats.hp { best = Some(i); }
    }
    best
}