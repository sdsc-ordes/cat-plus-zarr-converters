use serde_json::{Map, Value};

/// Extracts and separates data and metadata from a JSON object.
///
/// The keys used to identify the data are `data cube` and `peak`.
/// # Arguments
/// - `json`: A JSON object from which to extract data and metadata from.
/// - `data_map`: The map where the data metadata and data vectors will be stored.
/// - `metadata_map`: The map where the metadata will be stored.
pub fn extract_and_separate(
    json: &Value,
    data_map: &mut Map<String, Value>,
    metadata_map: &mut Map<String, Value>,
) -> Result<(), Box<dyn std::error::Error>> {
    match json {
        Value::Object(map) => {
            for (key, value) in map {
                if key.contains("data cube") || key.contains("peak") {
                    data_map.insert(key.clone(), value.clone());
                } else if value.is_object() || value.is_array() {
                    let mut nested_data = Map::new();
                    let mut nested_metadata = Map::new();
                    extract_and_separate(value, &mut nested_data, &mut nested_metadata)?;

                    if !nested_data.is_empty() {
                        data_map.insert(key.clone(), Value::Object(nested_data));
                    }
                    if !nested_metadata.is_empty() {
                        metadata_map.insert(key.clone(), Value::Object(nested_metadata));
                    }
                } else {
                    metadata_map.insert(key.clone(), value.clone());
                }
            }
        }

        Value::Array(array) => {
            let mut data_array = vec![];
            let mut metadata_array = vec![];

            for item in array {
                let mut nested_data = Map::new();
                let mut nested_metadata = Map::new();
                extract_and_separate(item, &mut nested_data, &mut nested_metadata)?;

                if !nested_data.is_empty() {
                    data_array.push(Value::Object(nested_data));
                }
                if !nested_metadata.is_empty() {
                    metadata_array.push(Value::Object(nested_metadata));
                }
            }

            if !data_array.is_empty() {
                *data_map = data_map.clone();
                data_map.insert("data".to_string(), Value::Array(data_array));
            }
            if !metadata_array.is_empty() {
                *metadata_map = metadata_map.clone();
                metadata_map.insert("metadata".to_string(), Value::Array(metadata_array));
            }
        }
        _ => {}
    }
    Ok(())
}
