/**
 * Auditable
 * @paulyhedral
 */
use std::option::Option;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// Base struct for auditable fields.
/// The auditable fields are meant to be used as a way of tracking the creation, update, and deletion of records.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Auditable {
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    pub created_at: DateTime<Utc>, // Not a reference for easier serialization
    pub created_by: String,
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    pub updated_at: DateTime<Utc>,
    pub updated_by: String,
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    pub deleted_at: Option<DateTime<Utc>>, // Option to represent nullable BSON fields
    pub deleted_by: Option<String>,
}

impl Auditable {
    pub fn new(created_by: String) -> Auditable {
        let now = Utc::now();
        Self {
            created_at: now,
            created_by: created_by.clone(),
            updated_at: now,
            updated_by: created_by.clone(),
            deleted_at: None,
            deleted_by: None,
        }
    }
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

        let now = chrono::Utc::now();

        let auditable = AuditableTest {
            some_field: "some value".to_string(),
            auditable: Auditable {
                created_at: now,
                created_by: "someone".to_string(),
                updated_at: now,
                updated_by: "someone-else".to_string(),
                deleted_at: None,
                deleted_by: None,
            },
        };

        assert_eq!(auditable.some_field, "some value");
        assert_eq!(auditable.auditable.created_at, now);
        assert_eq!(auditable.auditable.created_by, "someone");
        assert_eq!(auditable.auditable.updated_at, now);
        assert_eq!(auditable.auditable.updated_by, "someone-else");
        assert_eq!(auditable.auditable.deleted_at, None);
        assert_eq!(auditable.auditable.deleted_by, None);

        let json = serde_json::to_string(&auditable).unwrap();
        let auditable2: AuditableTest = serde_json::from_str(&json).unwrap();

        assert_eq!(auditable2.some_field, "some value");
        assert_eq!(auditable2.auditable.created_at, now);
        assert_eq!(auditable2.auditable.created_by, "someone");
        assert_eq!(auditable2.auditable.updated_at, now);
        assert_eq!(auditable2.auditable.updated_by, "someone-else");
        assert_eq!(auditable2.auditable.deleted_at, None);
        assert_eq!(auditable2.auditable.deleted_by, None);
    }
}
