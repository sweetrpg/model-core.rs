use serde::{Serialize, Deserialize};

/// A struct for tags.
/// The tag is a name/value pair, meant to be used as a way of labeling or categorizing things.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tag {
    pub name: String,
    pub value: String,
}

// ----------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Serialize, Deserialize};

    #[test]
    fn use_tag() {
        #[derive(Debug, Clone, Serialize, Deserialize)]
        struct TagTest {
            some_field: String,
            tag: Tag,
        }

        let tag = TagTest {
            some_field: "some value".to_string(),
            tag: Tag {
                name: "name".to_string(),
                value: "value".to_string(),
            },
        };

        assert_eq!(tag.some_field, "some value");
        assert_eq!(tag.tag.name, "name");
        assert_eq!(tag.tag.value, "value");

        let json = serde_json::to_string(&tag).unwrap();
        let tag2: TagTest = serde_json::from_str(&json).unwrap();

        assert_eq!(tag2.some_field, "some value");
        assert_eq!(tag2.tag.name, "name");
        assert_eq!(tag2.tag.value, "value");
    }
}
