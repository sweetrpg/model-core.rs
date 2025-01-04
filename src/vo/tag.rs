use serde::{Serialize, Deserialize};

/// A value object representing a tag.
/// The tag is a name/value pair, meant to be used as a way of labeling or categorizing things.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TagVO {
    pub name: String,
    pub value: String,
}

// ----------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Serialize, Deserialize};

    #[test]
    fn use_tag_vo() {
        #[derive(Debug, Clone, Serialize, Deserialize)]
        struct TagVOTest {
            some_field: String,
            tag: TagVO,
        }

        let vo = TagVOTest {
            some_field: "some value".to_string(),
            tag: TagVO {
                name: "name".to_string(),
                value: "value".to_string(),
            },
        };

        assert_eq!(vo.some_field, "some value");
        assert_eq!(vo.tag.name, "name");
        assert_eq!(vo.tag.value, "value");

        let json = serde_json::to_string(&vo).unwrap();
        let vo2: TagVOTest = serde_json::from_str(&json).unwrap();

        assert_eq!(vo2.some_field, "some value");
        assert_eq!(vo2.tag.name, "name");
        assert_eq!(vo2.tag.value, "value");
    }
}
