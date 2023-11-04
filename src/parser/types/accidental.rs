use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum Accidental {
    Sharp, // #
    Flat,  // b
}
