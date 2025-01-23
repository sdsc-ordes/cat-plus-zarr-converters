use std::{path::PathBuf, sync::Arc};
use zarrs::{filesystem::FilesystemStore, storage::ReadableWritableListableStorage};

/// Create a Zarr Store
///
/// # Arguments
/// - `store_path` : Path where to create the store
pub fn create_store(
    store_path: &PathBuf,
) -> Result<ReadableWritableListableStorage, Box<dyn std::error::Error>> {
    let filesystem = FilesystemStore::new(&store_path).expect("Failed to create filesystem store");

    let store: ReadableWritableListableStorage = Arc::new(filesystem);
    Ok(store)
}
