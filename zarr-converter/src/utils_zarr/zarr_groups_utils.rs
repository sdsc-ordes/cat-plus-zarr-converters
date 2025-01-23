use serde_json::{Map, Value};
use zarrs::{group::GroupBuilder, storage::ReadableWritableListableStorage};

/// Add a root group to the Zarr store
///
/// # Arguments
/// - `store`: The Zarr store where the root group will be added
pub fn add_root_group(
    store: &mut ReadableWritableListableStorage,
) -> Result<(), Box<dyn std::error::Error>> {
    GroupBuilder::new()
        .build(store.clone(), "/")?
        .store_metadata()?;
    Ok(())
}

/// Create a group in the Zarr store
///
/// # Arguments
/// - `store`: The Zarr store where the group will be added
/// - `group_path`: The path in the store where the group will be created
/// - `metadata`: The metadata to be associated with the group
pub fn create_group(
    store: &mut ReadableWritableListableStorage,
    group_path: &str,
    metadata: Map<String, Value>,
) -> Result<(), Box<dyn std::error::Error>> {
    let _group = zarrs::group::GroupBuilder::new()
        .attributes(metadata)
        .build(store.clone(), group_path)?;

    _group.store_metadata()?;
    Ok(())
}
