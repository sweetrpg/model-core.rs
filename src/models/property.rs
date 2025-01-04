use serde::{Serialize, Deserialize};

/// Struct for properties.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Property {
    pub name: String,
    pub kind: String,
    pub value: String,
}
