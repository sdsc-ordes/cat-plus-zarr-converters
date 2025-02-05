use sophia::inmem::graph::LightGraph;
use sophia_api::{
    graph::MutableGraph,
    term::{SimpleTerm, Term},
};

use super::utils::generate_bnode_term;

/// Used in [InsertIntoGraph::attach_and_insert].
#[derive(Clone)]
pub struct Link<'a, 'b, 'c> {
    pub source_iri: SimpleTerm<'a>,
    pub pred: SimpleTerm<'b>,
    pub target_iri: Option<SimpleTerm<'c>>,
}

/// InsertIntoGraph provides a trait to implement the conversion into a graph
/// by different types.
pub trait InsertIntoGraph {
    /// Inserts `&self` into `graph` with subject IRI `iri`
    fn insert_into(&self, graph: &mut LightGraph, iri: SimpleTerm) -> anyhow::Result<()>;

    /// Inserts `&self` into `graph` with subject IRI `iri` (default to a blank node)
    /// and "attach" self to an existing node with an additional triple.
    fn attach_into(&self, graph: &mut LightGraph, attach: Link) -> anyhow::Result<()> {
        let iri = attach.target_iri.unwrap_or_else(|| self.get_uri());
        _ = graph.insert(&attach.source_iri, &attach.pred, &iri);

        self.insert_into(graph, iri)
    }

    fn get_uri(&self) -> SimpleTerm<'static> {
        generate_bnode_term()
    }
}

/// Default implementation for [Option<T>].
impl<T> InsertIntoGraph for Option<T>
where
    T: InsertIntoGraph,
{
    fn insert_into(&self, archive: &mut LightGraph, iri: SimpleTerm) -> anyhow::Result<()> {
        if let Some(v) = self {
            v.insert_into(archive, iri)?
        }
        Ok(())
    }

    fn attach_into(&self, graph: &mut LightGraph, attach: Link) -> anyhow::Result<()> {
        if let Some(v) = self {
            v.attach_into(graph, attach)?
        }
        Ok(())
    }
}

impl<T> InsertIntoGraph for Vec<T>
where
    T: InsertIntoGraph,
{
    fn insert_into(&self, graph: &mut LightGraph, iri: SimpleTerm) -> anyhow::Result<()> {
        for item in self {
            item.insert_into(graph, iri.clone())?;
        }
        Ok(())
    }

    fn attach_into(&self, graph: &mut LightGraph, attach: Link) -> anyhow::Result<()> {
        for item in self {
            item.attach_into(graph, attach.clone())?;
        }
        Ok(())
    }
}

/// Default implementation for [SimpleTerm].
impl<'a> InsertIntoGraph for SimpleTerm<'a> {
    fn insert_into(&self, _graph: &mut LightGraph, _iri: SimpleTerm) -> anyhow::Result<()> {
        unimplemented!("cannot insert {:?} into graph, use `attach_and_insert`", &self)
    }

    fn attach_into(&self, graph: &mut LightGraph, attach: Link) -> anyhow::Result<()> {
        assert!(!self.is_triple());

        _ = graph.insert(&attach.source_iri, &attach.pred, self);

        Ok(())
    }
}
