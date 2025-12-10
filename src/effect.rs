use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Effect {
    Buff { stat: String, amount: i32, duration: u32 },
    Debuff { stat: String, amount: i32, duration: u32 },
    ExtraAction { actions: u32, duration: u32 },
    Passive { name: String },
}