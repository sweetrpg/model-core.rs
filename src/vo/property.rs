use serde::{Serialize, Deserialize};

/// Value object for properties.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PropertyVO {
    pub name: String,
    pub kind: String,
    pub value: String,
}
