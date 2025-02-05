use sophia_api::{
    prelude::*,
    term::{bnode_id::BnodeId, SimpleTerm},
};
use uuid::Uuid;

pub fn generate_bnode_term() -> SimpleTerm<'static> {
    let identifier = Uuid::new_v4().to_string();
    let bnode = BnodeId::new_unchecked(identifier);

    bnode.try_into_term().expect("Failed to convert BnodeId to SimpleTerm")
}
