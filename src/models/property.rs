use serde::{Serialize, Deserialize};

/// Struct for properties.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Property {
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
    fn use_property() {
        #[derive(Debug, Clone, Serialize, Deserialize)]
        struct PropertyTest {
            some_field: String,
            property: Property,
        }

        let property = PropertyTest {
            some_field: "some value".to_string(),
            property: Property {
                name: "name".to_string(),
                kind: "kind".to_string(),
                value: "value".to_string(),
            },
        };

        assert_eq!(property.some_field, "some value");
        assert_eq!(property.property.name, "name");
        assert_eq!(property.property.kind, "kind");
        assert_eq!(property.property.value, "value");

        let json = serde_json::to_string(&property).unwrap();
        let property2: PropertyTest = serde_json::from_str(&json).unwrap();

        assert_eq!(property2.some_field, "some value");
        assert_eq!(property2.property.name, "name");
        assert_eq!(property2.property.kind, "kind");
        assert_eq!(property2.property.value, "value");
    }
}
