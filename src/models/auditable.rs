use std::time::Instant;
use std::option::Option;

///
#[derive(Debug)]
pub struct Auditable {
    created_at: Instant,
    created_by: String,
    updated_at: Instant,
    updated_by: String,
    deleted_at: Option<Instant>,
    deleted_by: Option<String>,
}
