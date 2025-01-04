/**
 * Auditable
 * @paulyhedral
 */
use std::option::Option;
use serde::{Deserialize, Serialize};

/// Base struct for auditable fields.
/// The auditable fields are meant to be used as a way of tracking the creation, update, and deletion of records.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Auditable {
    pub created_at: u64,
    pub created_by: String,
    pub updated_at: u64,
    pub updated_by: String,
    pub deleted_at: Option<u64>,
    pub deleted_by: Option<String>,
}

// ----------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};

    #[test]
    fn use_auditable() {
        #[derive(Debug, Clone, Serialize, Deserialize)]
        struct AuditableTest {
            some_field: String,
            auditable: Auditable,
        }

        let auditable = AuditableTest {
            some_field: "some value".to_string(),
            auditable: Auditable {
                created_at: 0,
                created_by: "someone".to_string(),
                updated_at: 0,
                updated_by: "someone-else".to_string(),
                deleted_at: None,
                deleted_by: None,
            },
        };

        assert_eq!(auditable.some_field, "some value");
        assert_eq!(auditable.auditable.created_at, 0);
        assert_eq!(auditable.auditable.created_by, "someone");
        assert_eq!(auditable.auditable.updated_at, 0);
        assert_eq!(auditable.auditable.updated_by, "someone-else");
        assert_eq!(auditable.auditable.deleted_at, None);
        assert_eq!(auditable.auditable.deleted_by, None);

        let json = serde_json::to_string(&auditable).unwrap();
        let auditable2: AuditableTest = serde_json::from_str(&json).unwrap();

        assert_eq!(auditable2.some_field, "some value");
        assert_eq!(auditable2.auditable.created_at, 0);
        assert_eq!(auditable2.auditable.created_by, "someone");
        assert_eq!(auditable2.auditable.updated_at, 0);
        assert_eq!(auditable2.auditable.updated_by, "someone-else");
        assert_eq!(auditable2.auditable.deleted_at, None);
        assert_eq!(auditable2.auditable.deleted_by, None);
    }
}
