use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Effect {
    ReduceMovement(i32, i32), // amount, duration in turns
    Poison(i32, i32),         // damage per turn, duration in turns
}