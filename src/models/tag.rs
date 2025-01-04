use serde::{Serialize, Deserialize};

/// A struct for tags.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tag {
    pub name: String,
    pub value: String,
}
