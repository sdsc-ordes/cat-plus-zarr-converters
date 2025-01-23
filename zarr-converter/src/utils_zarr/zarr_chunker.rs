use ndarray::{s, ArrayBase, ArrayViewD, Dim, IxDynImpl};
use zarrs::{array::Array, storage::ReadableWritableListableStorage};

/// Chunk an ndarray and store the chunks in a Zarr array
///     
/// # Arguments
/// - `data` : The NDarray to be chunked into a Zarr array
/// - `chunk_shape` : The shape of the chunks
/// - `store` : The store where the Zarr array is stored
/// - `array_path` : The path in the store to the Zarr array
pub fn chunk(
    data: &ArrayBase<ndarray::OwnedRepr<f32>, Dim<IxDynImpl>>,
    chunk_shape: [usize; 2],
    store: &ReadableWritableListableStorage,
    array_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let shape = data.shape();
    for i in (0..shape[0]).step_by(chunk_shape[0]) {
        for j in (0..shape[1]).step_by(chunk_shape[1]) {
            let row_end = (i + chunk_shape[0]).min(shape[0]);
            let col_end = (j + chunk_shape[1]).min(shape[1]);
            let chunk_extraction: ArrayViewD<f32> =
                data.slice(s![i..row_end, j..col_end]).into_dyn();
            let chunk_elements: Vec<f32> = chunk_extraction.iter().cloned().collect();
            let expected_size = (row_end - i) * (col_end - j);
            if chunk_elements.len() != expected_size {
                return Err(format!(
                    "Chunk size mismatch: expected {}, got {}",
                    expected_size,
                    chunk_elements.len()
                )
                .into());
            }
            let chunk_indices = [(i / chunk_shape[0]) as u64, (j / chunk_shape[1]) as u64];
            add_chunk(&store, array_path, chunk_indices, chunk_elements)?;
        }
    }
    return Ok(());
}

/// Add a chunk to a Zarr array
///
/// # Arguments
/// - `store` : The store where the Zarr array is stored
/// - `array_path` : The path in the store to the Zarr array
/// - `chunk_indices` : The indices where the chunk should be added within the array
/// - `chunk_elements` : The elements of the chunk
fn add_chunk(
    store: &ReadableWritableListableStorage,
    array_path: &str,
    chunk_indices: [u64; 2],
    chunk_elements: Vec<f32>,
) -> Result<(), Box<dyn std::error::Error>> {
    let _array = Array::open(store.clone(), array_path)?;
    _array.store_chunk_elements::<f32>(&chunk_indices, &chunk_elements)?;
    Ok(())
}

/// Erase a chunk from a Zarr array
///
/// # Arguments
/// - `store` : The store where the Zarr array is stored
/// - `array_path` : The path in the store to the Zarr array
/// - `chunk_indices` : The indices of the chunk to be erased
pub fn erase_chunk(
    store: &mut ReadableWritableListableStorage,
    array_path: &str,
    chunk_indices: [u64; 2],
) -> Result<(), Box<dyn std::error::Error>> {
    let _array = Array::open(store.clone(), array_path)?;
    _array.erase_chunk(&chunk_indices)?;
    Ok(())
}
