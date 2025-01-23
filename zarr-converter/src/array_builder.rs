use ndarray::{stack, Array2, ArrayBase, Axis, Dim, IxDynImpl};
use serde_json::Value;

/// Convert a vector into a 2D NDarray
///
/// # Arguments
/// - `measures` - A vector of vectors of numbers
///
/// # Returns
/// A `Result` containing the 2D NDarray.
pub fn convert_to_2d_ndarray(
    measures: &Vec<Value>,
) -> Result<ArrayBase<ndarray::OwnedRepr<f32>, Dim<IxDynImpl>>, Box<dyn std::error::Error>> {
    let rows = measures.len();
    let cols = measures[0]
        .as_array()
        .ok_or("Expected 'measures[0]' to be an array")?
        .len();

    let mut flat_values: Vec<f32> = Vec::new();

    for row in measures {
        let row_array = row
            .as_array()
            .ok_or("Expected each element in 'measures' to be an array")?;
        for value in row_array {
            if let Some(num) = value.as_f64() {
                flat_values.push(num as f32);
            } else {
                return Err("Expected all elements in 'measures' to be numbers".into());
            }
        }
    }
    if flat_values.len() != rows * cols {
        return Err("Mismatch in measures dimensions or missing values".into());
    }
    let array_2d = Array2::from_shape_vec((rows, cols), flat_values)?;
    let array_dyn = array_2d.into_dyn();
    Ok(array_dyn)
}

/// Build a 2D NDarray from two vectors of numbers
///
/// # Arguments
/// - `measures` - A vector of numbers
/// - `dimensions` - A vector of numbers
///
/// # Returns
/// A `Result` containing the 2D NDarray.
pub fn build_2d_ndarray(
    measures: &Vec<Value>,
    dimensions: &Vec<Value>,
) -> Result<ArrayBase<ndarray::OwnedRepr<f32>, Dim<IxDynImpl>>, Box<dyn std::error::Error>> {
    if measures.len() != dimensions.len() {
        return Err("measures and dimensions vectors must have the same length".into());
    }

    let parsed_measures: Vec<f32> = measures
        .iter()
        .filter_map(|v| v.as_f64().map(|f| f as f32))
        .collect();
    let parsed_dimensions: Vec<f32> = dimensions
        .iter()
        .filter_map(|v| v.as_f64().map(|f| f as f32))
        .collect();

    if parsed_measures.len() != measures.len() || parsed_dimensions.len() != dimensions.len() {
        return Err("Failed to parse all values into f32".into());
    }

    let measures_array = ndarray::Array1::from_vec(parsed_measures);
    let dimensions_array = ndarray::Array1::from_vec(parsed_dimensions);

    let array_2d = stack(Axis(1), &[measures_array.view(), dimensions_array.view()])?;
    let array_dyn = array_2d.into_dyn();
    Ok(array_dyn)
}
