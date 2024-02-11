use serde::{Deserialize, Serialize};

use crate::types::FType;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Mutation {
    AddAffliction(String, FType),
    RemoveAffliction(String, FType),
}
