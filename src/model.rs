use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Index {
    #[serde(rename = "mappings", default)]
    mapping: Mapping,
}

#[derive(Default, Deserialize, Serialize)]
struct Mapping {
    #[serde(default)]
    properties: HashMap<String, Property>,
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct Property {
    #[serde(rename = "type")]
    kind: MappingType,
}

// https://www.elastic.co/guide/en/elasticsearch/reference/current/mapping-types.html
#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
enum MappingType {
    Boolean,
    Date,
    Keyword,
    Text,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serde_with_field_mapping() {
        let json = r#"{"mappings":{"properties":{"user_id":{"type":"keyword"}}}}"#;
        let index = serde_json::from_str::<Index>(json).unwrap();
        assert!(index.mapping.properties.get("user_id").is_some());
        assert_eq!(
            index.mapping.properties.get("user_id").unwrap().kind,
            MappingType::Keyword
        );
    }

    #[test]
    fn test_serde_with_unsupported_field_mappings() {
        let json = r#"{"mappings":{"properties":{"user_id":{"type":"keyword","index":true}}}}"#;
        assert!(serde_json::from_str::<Index>(json).is_err());
    }

    #[test]
    fn test_serde_with_empty_properties_object() {
        let json = r#"{"mappings":{"properties":{}}}"#;
        let index = serde_json::from_str::<Index>(json).unwrap();
        assert!(index.mapping.properties.is_empty());
    }

    #[test]
    fn test_serde_with_omitted_properties_object() {
        let json = r#"{"mappings":{}}"#;
        let index = serde_json::from_str::<Index>(json).unwrap();
        assert!(index.mapping.properties.is_empty());
    }

    #[test]
    fn test_serde_with_omitted_mappings_object() {
        let json = r#"{}"#;
        let index = serde_json::from_str::<Index>(json).unwrap();
        assert!(index.mapping.properties.is_empty());
    }
}
