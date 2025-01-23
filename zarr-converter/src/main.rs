use serde_json::{Map, Value};
use std::{collections::HashMap, fs, io::BufWriter, path::PathBuf};

use clap::Parser;
use std::{fs::File, path::Path};

use zarr_converter::{
    agilent::agilent_get_data,
    extractor::extract_and_separate,
    utils::find_middle_divisor,
    utils_zarr::{
        zarr_arrays_utils::{create_array, retrieve_ndarray},
        zarr_chunker::chunk,
        zarr_groups_utils::{add_root_group, create_group},
        zarr_metadata_consolidate::collect_metadata,
        zarr_store_utils::create_store,
    },
};

#[derive(Parser, Debug)]
struct Args {
    input_file: String,
    group_path: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // ZARR: CREATE STORE
    let store_path: PathBuf = "../zarr-files/hierarchy.zarr".into();
    let mut store = create_store(&store_path)?;
    add_root_group(&mut store)?;

    // Process Args
    let args = Args::parse();
    let input_file = Path::new(&args.input_file);
    let group_path = &args.group_path;

    // Process Input file
    let file_content = fs::read_to_string(input_file)?;
    let json: Value = serde_json::from_str(&file_content)?;

    // Seperate and extract data from metadata
    let mut data_map = Map::new();
    let mut metadata_map = Map::new();
    extract_and_separate(&json, &mut data_map, &mut metadata_map)?;

    // ZARR: CREATE GROUP & its METADATA
    let group_metadata = metadata_map.clone();
    create_group(&mut store, group_path, group_metadata)?;

    // AGILENT DATA CUBES
    let mut data_cubes = HashMap::new();
    agilent_get_data(&data_map, &mut data_cubes)?;

    // ZARR: ARRAYS
    // - Create new Zarr array
    // - Define ideal chunk shape
    // - Chunk data cube into the Zarr array

    for (data_cube_name, data_cube) in data_cubes.iter() {
        // Calculate ideal chunk shape for that data cube
        let chunk_shape = find_middle_divisor(data_cube.shape());

        // Zarr: CREATE ARRAY & its METADATA
        let array_path = format!("/agilent/_{}", data_cube_name);
        let array_shape = data_cube
            .shape()
            .to_vec()
            .iter()
            .map(|&x| x as u64)
            .collect();
        let dimension_names = vec!["y", "x"];
        let chunking_vec: Vec<u64> = vec![chunk_shape.try_into().unwrap(), 2];
        let json_metadata = serde_json::json!({"Zarr V3": "is great"})
            .as_object()
            .unwrap()
            .clone();
        create_array(
            &mut store,
            &array_path,
            array_shape,
            chunking_vec,
            dimension_names,
            json_metadata,
        )?;

        // Zarr: CHUNK DATA CUBE into ARRAY
        println!("Chunking Data Cube into Zarr Array: {}", data_cube_name);
        println!("Data Cube size: {:?}", data_cube.shape());
        let chunking_usize: [usize; 2] = [chunk_shape, 2];
        chunk(&data_cube, chunking_usize, &store, &array_path)?;

        // Zarr: RETRIEVE Full Zarr ARRAY to NDARRAY and check shape
        let array_ndarray = retrieve_ndarray(&mut store, &array_path)?;
        println!("Retrieving array from Zarr: {:?}", array_ndarray.shape());
    }

    // CONSOLIDATE METADATA
    let consolidated_metadata = collect_metadata(&mut store)?;

    // Write all consolidated metadata to a JSON
    let data_file = File::create("data/consolidated_metadata.json")?;
    let writer = BufWriter::new(data_file);
    serde_json::to_writer_pretty(writer, &consolidated_metadata)?;
    println!("Data written to consolidated_metadata.json");

    return Ok(());
}
