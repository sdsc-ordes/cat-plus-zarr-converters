use lazy_static::lazy_static;
use sophia::api::ns::Namespace;
use sophia_api::namespace;

// Rust cannot handle dashes in variable names, therefore this
// namespace uses rust adapted term names and then maps them on output
// to the real ontology units
namespace! {
    "https://qudt.org/vocab/unit/",
    Bar,
    DegC,
    MilliGM,
    GMPerMilliL,
    GMPerMol,
    MolPerL,
    RevPerMin
}

lazy_static! {
    pub static ref ns: Namespace<&'static str> =
        Namespace::new("https://qudt.org/vocab/unit/").unwrap();
}
