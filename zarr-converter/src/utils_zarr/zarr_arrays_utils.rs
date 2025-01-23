use serde_json::{Map, Value};
use std::sync::Arc;
use zarrs::{
    array::{codec::GzipCodec, Array, ArrayBuilder, DataType, FillValue, ZARR_NAN_F32},
    storage::ReadableWritableListableStorage,
};

/// Create Zarr Array
///
/// # Arguments
/// - `store`: The store in which the array will be added
/// - `array_path`: The path within the store where the array should be created
/// - `array_shape`: The shape of the array
/// - `chunking_shape`: The shape of the chunks in which the array will be stored
/// - `dimension_names`: The names of the dimensions of the array
/// - `metadata`: The metadata to be added to the array
pub fn create_array(
    store: &mut ReadableWritableListableStorage,
    array_path: &str,
    array_shape: Vec<u64>,
    chunking_shape: Vec<u64>,
    dimension_names: Vec<&str>,
    metadata: Map<String, Value>,
) -> Result<(), Box<dyn std::error::Error>> {
    let _array = ArrayBuilder::new(
        array_shape,
        DataType::Float32,
        chunking_shape.try_into()?,
        FillValue::from(ZARR_NAN_F32),
    )
    .bytes_to_bytes_codecs(vec![Arc::new(GzipCodec::new(5)?)])
    .dimension_names(dimension_names.into())
    .attributes(metadata)
    .build(store.clone(), array_path)?;
    _array.store_metadata()?;
    Ok(())
}

/// Retrieves the entire Array from a Zarr store and returns it as an NDarray
///
/// # Arguments
/// - `store`: The store in which the array is stored
/// - `array_path`: The path within the store where the array is stored
///
/// # Returns
/// A `Result` containing the array formatted as an NDarray
pub fn retrieve_ndarray(
    store: &mut ReadableWritableListableStorage,
    array_path: &str,
) -> Result<ndarray::ArrayD<f32>, Box<dyn std::error::Error>> {
    let _array = Array::open(store.clone(), array_path)?;
    let array_ndarray = _array.retrieve_array_subset_ndarray::<f32>(&_array.subset_all())?;
    Ok(array_ndarray)
}
