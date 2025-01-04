use std::option::Option;
use serde::{Serialize, Deserialize};

/// A value object representing auditable fields.
/// The auditable fields are meant to be used as a way of tracking the creation, update, and deletion of records.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditableVO {
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
    use serde::{Serialize, Deserialize};
    use serde_json;

    #[test]
    fn use_auditable_vo() {
        #[derive(Debug, Clone, Serialize, Deserialize)]
        struct AuditableVOTest {
            some_field: String,
            auditable: AuditableVO,
        }

        let vo = AuditableVOTest {
            some_field: "some value".to_string(),
            auditable: AuditableVO {
                created_at: 0,
                created_by: "someone".to_string(),
                updated_at: 0,
                updated_by: "someone-else".to_string(),
                deleted_at: None,
                deleted_by: None,
            },
        };

        assert_eq!(vo.some_field, "some value");
        assert_eq!(vo.auditable.created_at, 0);
        assert_eq!(vo.auditable.created_by, "someone");
        assert_eq!(vo.auditable.updated_at, 0);
        assert_eq!(vo.auditable.updated_by, "someone-else");
        assert_eq!(vo.auditable.deleted_at, None);
        assert_eq!(vo.auditable.deleted_by, None);

        let json = serde_json::to_string(&vo).unwrap();
        let vo2: AuditableVOTest = serde_json::from_str(&json).unwrap();

        assert_eq!(vo2.some_field, "some value");
        assert_eq!(vo2.auditable.created_at, 0);
        assert_eq!(vo2.auditable.created_by, "someone");
        assert_eq!(vo2.auditable.updated_at, 0);
        assert_eq!(vo2.auditable.updated_by, "someone-else");
        assert_eq!(vo2.auditable.deleted_at, None);
        assert_eq!(vo2.auditable.deleted_by, None);
    }
}
