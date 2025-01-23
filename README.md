# Cat+ Zarr Converters

## About

This repository contains all the Zarr converters for the different data types in the Cat+ project (Agilent, UV, IR, etc.)
The data types are all in different formats, their data and metadata colluded together. The goal will be to convert the metadata to [an established ontology](https://github.com/sdsc-ordes/cat-plus-ontology/tree/main), and -as much as data format allow- convert the data in [Zarr array](https://zarr.readthedocs.io/en/stable/index.html).

## Tools: RDF Converters

### synth-converter
The Synth-converter parses a json input into an rdf graph and serializes the graph to either turtle or jsonld.
It expects the input to conform to the cat+ ontology and the struct `synth-converter/src/batch.rs`. An example input file is provided in `example/1-Synth.json`.

#### Usage

The `synth-converter` has three parameters:

- inputfile: path to input file (relative to top level of the repo or absolute)
- outputfile: path to output file (relative to top level of the repo or absolute)
- format: default is "ttl", the other option is jsonld

The `synth-converter` turns the inputfile into a rdf graph and serilizes it to either turtle or jsonld. The serialization is written to an outputfile.

```
just run example/1-Synth.json output.ttl
just run example/1-Synth.json output.json --format jsonld
```

### Architecture

The json input is read with `serde_json`: the transformation of fields is described in the struct `synth-converter/src/batch.rs`

The graph is build via `synth-converter/src/graph/graph_builder.rs` and uses `sophia_rs`. Besides `rdf` and `xsd` that have build in namespaces in `sophia_rs`, all namespaces and terms are provided in `synth-converter/src/graph/namespaces` as constants. This makes the code more readable and also ensures that the rdf iris and namespaces are controlled and spelt correctly.
Graph serializers and parsers are provided in `synth-converter/src/rdf`. The turtle serializer there is needed for the test.
The conversion is done in the public crate `synth-converter/src/convert.rs`

### Shacl Validation

The rdf graph confirms to the cat+ ontology: https://github.com/sdsc-ordes/cat-plus-ontology. Currently rust offeres no Shacl Validation Library, but once such a library exists, it would make sense to add a Shacl Validation.

The SHACL Validation can be done manually here: https://www.itb.ec.europa.eu/shacl/any/upload

## Tools: ZARR Converters

### zarr-converter

The Zarr converter has been developed (as first draft) as a CLI to handle the conversion of an Agilent data file (JSON) into a Zarr array. The file is first processed, to extract the data from the metadata. The data is then transformed into NDarrays. In future works, the metadata will be converted to RDF via an `agilent-converter` (see RDF Converters). 

##### Example Usage

```
cd zarr-converter
cargo run data/agilent-example.json /agilent
```
The Agilent JSON file will be placed in the `data` folder. In this same folder, the `consolidated_metadata.json` will be generated (for all the metadata in the Zarr Store). The Zarr hierarchy and zarr files will be generated in the `zarr-files` folder.

##### Road Map 

- Units Tests
- Allow a Store with multigroup 
- Decide on how to process the 3D data from Agilent
- Create an extractor to separate data metadata from data (current status: only separate high level metadata from data (data metadata & data))
- Adapt conversion to other file types : UV, IR (infrared), and NMR data

## Installation guidelines

The repo is setup with nix.

```
git clone git@github.com:sdsc-ordes/cat-plus-zarr-converters.git
cd cat-plus-zarr-converters
cargo build
```

From here on you can work with a just file:

The rust commands can be started via a justfile:

```
just --list
Available recipes:
    build *args                      # Build the synth-converter.
    default                          # Default recipe to list all recipes.
    nix-develop *args                # Enter a Nix development shell.
    run input_file output_file *args # Run the synth-converter.
    test *args                       # Test the synth-converter.
    fmt *arg                         # Format the synth-converter.
```

### Tests

Run the tests with `just test`: only integration tests have been integrated that ensure that the serialized graph in turtle is isomorphic to an expected turtle serialization per valid substructure of the input data: this substructures are action that occur in the synthesis process.

### Contribute

The repo is a Poc under heavy development and not yet ready to take contributions.
