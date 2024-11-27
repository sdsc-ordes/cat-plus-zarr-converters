use lazy_static::lazy_static;
use sophia::api::ns::Namespace;
use sophia_api::{prefix::Prefix, prelude::Iri};

pub mod ex {
    use lazy_static::lazy_static;
    use sophia::api::ns::Namespace;
    use sophia_api::namespace;

    namespace! {
      "http://example.org/ns#",
      Foo,
      Bar,
      Baz
    }

    lazy_static! {
        pub static ref ns: Namespace<&'static str> = Namespace::new(PREFIX.as_str()).unwrap();
    }
}

lazy_static! {
    pub static ref RDF: Namespace<&'static str> =
        Namespace::new("http://www.w3.org/1999/02/22-rdf-syntax-ns#").unwrap();
    pub static ref CAT: Namespace<&'static str> =
        Namespace::new("http://example.org/cat#").unwrap();
    pub static ref SCHEMA: Namespace<&'static str> = Namespace::new("https://schema.org/").unwrap();
    pub static ref ALLORES: Namespace<&'static str> =
        Namespace::new("http://purl.allotrope.org/ontologies/result#").unwrap();
    pub static ref EX: Namespace<&'static str> = Namespace::new("http://example.org/").unwrap();
    pub static ref QUDT: Namespace<&'static str> =
        Namespace::new("http://qudt.org/schema/qudt/").unwrap();
    pub static ref ALLOQUAL: Namespace<&'static str> =
        Namespace::new("http://purl.allotrope.org/ontologies/quality#").unwrap();
    pub static ref PURL: Namespace<&'static str> =
        Namespace::new("http://purl.allotrope.org/ontologies/").unwrap();
    pub static ref OBO: Namespace<&'static str> =
        Namespace::new("http://purl.obolibrary.org/obo/").unwrap();
    pub static ref XSD: Namespace<&'static str> =
        Namespace::new("http://www.w3.org/2001/XMLSchema#").unwrap();
}

pub fn generate_prefix_map() -> Vec<(Prefix<Box<str>>, Iri<Box<str>>)> {
    vec![
        ("rdf", RDF.get("").unwrap()),
        ("cat", CAT.get("").unwrap()),
        ("schema", SCHEMA.get("").unwrap()),
        ("allores", ALLORES.get("").unwrap()),
        ("ex", EX.get("").unwrap()),
        ("qudt", QUDT.get("").unwrap()),
        ("alloqual", ALLOQUAL.get("").unwrap()),
        ("purl", PURL.get("").unwrap()),
        ("obo", OBO.get("").unwrap()),
        ("xsd", XSD.get("").unwrap()),
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
