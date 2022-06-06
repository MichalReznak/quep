use crate::args::types::CircuitSchemaType;
use typed_builder::TypedBuilder;

#[derive(TypedBuilder, Clone)]
pub struct GenCircuit {
    pub circuit: String,
    pub t: CircuitSchemaType,
}
