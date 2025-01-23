use ndarray::ArrayD;
use serde_json::{Map, Value};
use std::collections::HashMap;

use crate::array_builder::{build_2d_ndarray, convert_to_2d_ndarray};

/// Formatting the Ultraviolet Data Cube of the Agilent data into an NDarray
///
/// # Arguments
/// - `data_cubes_extracted` - HashMap of all extracted data cubes
/// - `i` - Index of the data cube being formatted
/// - `data_cube` - Data cube being formatted
fn add_ultraviolet_cube(
    data_cubes_extracted: &mut HashMap<String, ArrayD<f32>>,
    i: usize,
    data_cube: &Value,
) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(variables) = data_cube
        .get("three-dimensional ultraviolet spectrum data cube")
        .and_then(|cube| cube.get("data"))
    {
        let measures = variables["measures"][0]
            .as_array()
            .ok_or("Expected 'data' to be an array")?;
        let ndarray = convert_to_2d_ndarray(measures)?;
        // QUESTION: What to do with the dimensions? Put in a metadata ? Have metadata & dimensions in attributes?
        //println!("NDarray: {:?}", ndarray.shape());
        data_cubes_extracted.insert(format!("data_cube_{}", i), ndarray);
    }
    return Ok(());
}

/// Formatting a Chromatogram Data Cube of the AGILENT data into an NDarray
///
/// # Arguments
/// - `data_cubes_extracted` - HashMap of all extracted data cubes
/// - `i` - Index of the data cube being formatted
/// - `data_cube` - Data cube being formatted
fn add_data_cube(
    data_cubes_extracted: &mut HashMap<String, ArrayD<f32>>,
    i: usize,
    data_cube: &Value,
) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(_check) = data_cube["chromatogram data cube"]["data"]["measures"].as_array() {
        let measures = data_cube["chromatogram data cube"]["data"]["measures"][0]
            .as_array()
            .ok_or("Expected 'data' to be an array")?;
        let dimensions = data_cube["chromatogram data cube"]["data"]["dimensions"][0]
            .as_array()
            .ok_or("Expected 'data' to be an array")?;
        let ndarray = build_2d_ndarray(measures, dimensions)?;
        data_cubes_extracted.insert(format!("data_cube_{}", i), ndarray);
    }
    return Ok(());
}

/// Formatting all the Data Cubes in the AGILENT data
///
/// Each data cube is formatted into an NDarray and stored in a HashMap.
/// A data cube can be a Chromatogram Data Cube, an Ultraviolet Data Cube, or a Mass Spectrum Data Cube.
///
/// # Arguments
/// - `data_map` - Map of the data metadata and data cubes extracted from the AGILENT data
/// - `data_cubes_extracted` - HashMap of all extracted data cubes
pub fn agilent_get_data(
    data_map: &Map<String, Value>,
    data_cubes_extracted: &mut HashMap<String, ArrayD<f32>>,
) -> Result<(), Box<dyn std::error::Error>> {
    let data_cubes = data_map["liquid chromatography aggregate document"]
        ["liquid chromatography document"]["data"][0]["measurement aggregate document"]
        ["measurement document"]["data"]
        .as_array()
        .ok_or("Expected 'data' to be an array")?;
    for (i, data_cube) in data_cubes.iter().enumerate() {
        add_data_cube(data_cubes_extracted, i, data_cube)?;
        //add_ultraviolet_cube(data_cubes_extracted, i, data_cube)?;
        //add_mass_cube(data_cubes_extracted, i, data_cube)?;
    }
    return Ok(());
}
