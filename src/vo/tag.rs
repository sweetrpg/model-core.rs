use serde::{Serialize, Deserialize};

/// A value object representing a tag.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TagVO {
    name: String,
    value: String,
}
