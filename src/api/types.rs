use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Random {
    pub message: String,
    pub status: String,
}
