use zarrs::{array::Array, group::Group};

use zarrs::storage::ReadableWritableListableStorage;

use serde_json::{json, Map, Value};

/// Add metadata to the all the metadata
///
/// # Arguments
/// - `all_metadata`: A JSON object containing all the metadata of the store where the new metadata will be added
/// - `name`: The name of object  for which the metadata will be added
/// - `metadata`: The metadata to be added
fn add_metadata<'all_metadata>(
    all_metadata: &'all_metadata mut Map<String, Value>,
    name: &str,
    metadata: Map<String, Value>,
) -> Result<(), Box<dyn std::error::Error>> {
    all_metadata.insert(name.to_string(), json!(metadata.clone()));
    Ok(())
}

/// Iterate over all groups in the store and collect metadata
///
/// # Arguments
/// - `store`: The store to collect metadata from
/// - `group_path`: The path of the group to iterate over (find child groups and get their metadata)
/// - `all_metadata`: A JSON object containing all the metadata of the store where the new metadata will be added
fn iterate_groups<'all_metadata>(
    store: ReadableWritableListableStorage,
    group_path: &str,
    all_metadata: &'all_metadata mut Map<String, Value>,
) -> Result<(), Box<dyn std::error::Error>> {
    let group = Group::open(store.clone(), group_path)?;
    println!("{}", group_path);
    let children_paths = group.child_group_paths(true)?;
    println!("{}", "children paths retrieved");
    println!("{:?}", children_paths);
    for child_path in children_paths {
        println!("{}", child_path);
        let child_group = Group::open(store.clone(), &child_path.to_string())?;
        let group_attrs = child_group.attributes().clone();
        add_metadata(all_metadata, &child_path.to_string(), group_attrs)?;
        iterate_groups(store.clone(), &child_path.to_string(), all_metadata)?;
        iterate_arrays(store.clone(), &child_path.to_string(), all_metadata)?;
    }
    Ok(())
}

/// Iterate over all arrays in the store and collect metadata
///
/// # Arguments
/// - `store`: The store to collect metadata from
/// - `group_path`: The path of the group to iterate over (find child arrays and get their metadata)
/// - `all_metadata`: A JSON object containing all the metadata of the store where the new metadata will be added
fn iterate_arrays<'all_metadata>(
    store: ReadableWritableListableStorage,
    group_path: &str,
    all_metadata: &'all_metadata mut Map<String, Value>,
) -> Result<(), Box<dyn std::error::Error>> {
    let group = Group::open(store.clone(), group_path)?;
    let children_paths = group.child_array_paths(true)?;
    for child_path in children_paths {
        println!("{}", child_path);
        let child_group = Array::open(store.clone(), &child_path.to_string())?;
        let group_attrs = child_group.attributes().clone();
        add_metadata(all_metadata, &child_path.to_string(), group_attrs)?;
    }
    Ok(())
}

/// Collect metadata for all groups and arrays in the store
///
/// # Arguments
/// - `store`: The store to collect metadata from
///
/// # Returns
/// A `Result` containing a JSON object containing the metadata for all groups and arrays in the store
pub fn collect_metadata(
    store: &mut ReadableWritableListableStorage,
) -> Result<Value, Box<dyn std::error::Error>> {
    // Iterate over the entire store and collect metadata for all groups and arrays
    let root_path = "/";
    let root_group = Group::open(store.clone(), root_path)?;
    let mut all_metadata = serde_json::Map::new();

    // Collect metadata for the root group itself
    let group_attrs: serde_json::Map<std::string::String, Value> = root_group.attributes().clone();
    add_metadata(&mut all_metadata, "root", group_attrs)?;
    //Collect the metadata in all nested groups and arrays
    iterate_groups(store.clone(), root_path, &mut all_metadata)?;
    iterate_arrays(store.clone(), root_path, &mut all_metadata)?;

    Ok(json!(all_metadata))
}
