use std::time::Instant;
use std::option::Option;

/// A value object representing auditable fields.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditableVO {
    pub created_at: Instant,
    pub created_by: String,
    pub updated_at: Instant,
    pub updated_by: String,
    pub deleted_at: Option<Instant>,
    pub deleted_by: Option<String>,
}
