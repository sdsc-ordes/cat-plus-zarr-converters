use crate::rdf::rdf_serializers::{serialize_graph_to_jsonld, serialize_graph_to_turtle};
use anyhow::{Context, Result};
use sophia::inmem::graph::LightGraph;

use super::insert_into::InsertIntoGraph;

/// An RDF Graph
pub struct GraphBuilder {
    pub graph: LightGraph,
}

/// Builds an RDF graph of Synthesis data for the cat+ ontology.
///
/// The rust structure `actions` in /parser/actions is mapped to the cat+ ontology
///
/// # public methods:
/// * insert:  starts the process of building the graph from the input structure
/// * serialize_to_turtle: serializes the graph to a turtle output
impl GraphBuilder {
    pub fn new() -> Self {
        Self { graph: LightGraph::new() }
    }

    /// Inserts a new object into the graph as a collection of triples.
    pub fn insert(&mut self, other: &dyn InsertIntoGraph) -> Result<()> {
        other.insert_into(&mut self.graph, other.get_uri())?;

        Ok(())
    }

    /// Get the turtle serialization of the RDF graph
    ///
    /// Assumes a new graph has been created and built.
    ///
    /// # Returns
    /// A `Result` containing the graph as Turtle serialization, or an error
    /// if the graph retrieval fails.
    pub fn serialize_to_turtle(&self) -> Result<String> {
        serialize_graph_to_turtle(&self.graph).context("Failed to serialize graph to Turtle")
    }

    /// Get the turtle serialization of the RDF graph
    ///
    /// Assumes a new graph has been created and built.
    ///
    /// # Returns
    ///  The `jsonld` serialization of the grap, or an error otherwise.
    /// if the graph retrieval fails.
    pub fn serialize_to_jsonld(&self) -> Result<String> {
        serialize_graph_to_jsonld(&self.graph).context("Failed to serialize graph to JSON-LD")
    }
}
