use serde_json::{Map, Value};

/// Trait for converting snake_case string to PascalCase
pub trait ToPascalCase {
    fn to_pascal_case(&self) -> String;
}

impl ToPascalCase for str {
    fn to_pascal_case(&self) -> String {
        self.split('_')
            .map(|word| {
                let mut chars = word.chars();
                match chars.next() {
                    Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
                    None => "".to_string(),
                }
            })
            .collect()
    }
}

/// Trait to transform all keys of a JSON object
pub trait TransformJsonKeys {
    fn transform_keys(&self, prefix: &str) -> Value;
}

impl TransformJsonKeys for Value {
    fn transform_keys(&self, prefix: &str) -> Value {
        match self {
            Value::Object(map) => {
                let mut new_map = Map::new();
                for (key, value) in map {
                    let pascal_key = key.to_pascal_case();
                    let new_key = format!("{}{}", prefix, pascal_key);
                    new_map.insert(new_key, value.clone());
                }
                Value::Object(new_map)
            }
            _ => self.clone(),
        }
    }
}