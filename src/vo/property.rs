/**
 * Property value object.
 * @paulyhedral
 */
use serde::{Serialize, Deserialize};

/// Value object for properties.
/// Properties are name/kind/value triplets, meant to be used as a way of adding arbitrary data
/// to other objects.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PropertyVO {
    pub name: String,
    pub kind: String,
    pub value: String,
}

// ----------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Serialize, Deserialize};

    #[test]
    fn use_property_vo() {
        #[derive(Debug, Clone, Serialize, Deserialize)]
        struct PropertyVOTest {
            some_field: String,
            property: PropertyVO,
        }

        let vo = PropertyVOTest {
            some_field: "some value".to_string(),
            property: PropertyVO {
                name: "name".to_string(),
                kind: "kind".to_string(),
                value: "value".to_string(),
            },
        };

        assert_eq!(vo.some_field, "some value");
        assert_eq!(vo.property.name, "name");
        assert_eq!(vo.property.kind, "kind");
        assert_eq!(vo.property.value, "value");

        let json = serde_json::to_string(&vo).unwrap();
        let vo2: PropertyVOTest = serde_json::from_str(&json).unwrap();

        assert_eq!(vo2.some_field, "some value");
        assert_eq!(vo2.property.name, "name");
        assert_eq!(vo2.property.kind, "kind");
        assert_eq!(vo2.property.value, "value");
    }
}
