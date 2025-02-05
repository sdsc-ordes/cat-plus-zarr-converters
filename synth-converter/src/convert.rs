use crate::{graph::graph_builder::GraphBuilder, models::Batch};
use anyhow::{Context, Result};

/// Parse JSON and serialize the RDF graph to the specified format
///
/// The input JSON should conform to the structure defined in the `Batch` struct.
/// An example input file, `1-Synth.json`, is available in the `example` directory.
///
/// # Arguments
/// - `input_content`: The JSON input as a string.
/// - `fmt`: The desired serialization format ("turtle" or "jsonld").
///           If unspecified or empty, defaults to "turtle".
///
/// # Returns
/// A `Result` containing the serialized graph as a string or an error if the process fails.
pub fn json_to_rdf(input_content: &str, fmt: &str) -> Result<String> {
    // Parse JSON into a Batch object
    let batch = parse_json(input_content).context("Failed to parse JSON input")?;

    // Build the RDF graph
    let mut graph_builder = GraphBuilder::new();
    graph_builder.insert(&batch).context("Failed to build RDF graph")?;

    // Serialize the RDF graph to the specified format
    let serialized_graph = match fmt {
        "jsonld" => graph_builder
            .serialize_to_jsonld()
            .context("Failed to serialize RDF graph to JSON-LD")?,
        _ => graph_builder
            .serialize_to_turtle()
            .context("Failed to serialize RDF graph to Turtle")?,
    };

    Ok(serialized_graph)
}

/// Parses a JSON string into a `Batch` struct
///
/// # Arguments
/// - `json_data`: The JSON data as a string.
///
/// # Returns
/// A `Result` containing the parsed `Batch` struct or an error.
fn parse_json(json_data: &str) -> Result<Batch> {
    serde_json::from_str(json_data).map_err(|e| anyhow::Error::new(e))
}
