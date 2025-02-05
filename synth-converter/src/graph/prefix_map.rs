use crate::graph::namespaces::{alloproc, alloqual, allores, cat, obo, purl, qudt, schema, unit};
use sophia_api::{prefix::Prefix, prelude::Iri};

use lazy_static::lazy_static;
use sophia::api::ns::Namespace;

lazy_static! {
    pub static ref rdf: Namespace<&'static str> =
        Namespace::new("http://www.w3.org/1999/02/22-rdf-syntax-ns#").unwrap();
    pub static ref xsd: Namespace<&'static str> =
        Namespace::new("http://www.w3.org/2001/XMLSchema#").unwrap();
}

pub fn generate_prefix_map() -> Vec<(Prefix<Box<str>>, Iri<Box<str>>)> {
    vec![
        ("rdf", rdf.get("").expect("Namespace URI should always be valid")),
        ("cat", cat::ns.get("").expect("Namespace URI should always be valid")),
        ("schema", schema::ns.get("").expect("Namespace URI should always be valid")),
        ("unit", unit::ns.get("").expect("Namespace URI should always be valid")),
        ("allores", allores::ns.get("").expect("Namespace URI should always be valid")),
        ("alloproc", alloproc::ns.get("").expect("Namespace URI should always be valid")),
        ("qudt", qudt::ns.get("").expect("Namespace URI should always be valid")),
        ("alloqual", alloqual::ns.get("").expect("Namespace URI should always be valid")),
        ("purl", purl::ns.get("").expect("Namespace URI should always be valid")),
        ("obo", obo::ns.get("").expect("Namespace URI should always be valid")),
        ("xsd", xsd.get("").expect("Namespace URI should always be valid")),
    ]
    .into_iter()
    .map(|(prefix, iri)| {
        (
            Prefix::new(prefix.to_string().into_boxed_str()).expect("Invalid prefix"),
            Iri::new(iri.to_string().into_boxed_str()).expect("Invalid IRI"),
        )
    })
    .collect()
}
