use std::option::Option;
use serde::{Serialize, Deserialize};

/// Base struct for auditable fields.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Auditable {
    pub created_at: u64,
    pub created_by: String,
    pub updated_at: u64,
    pub updated_by: String,
    pub deleted_at: Option<u64>,
    pub deleted_by: Option<String>,
}
