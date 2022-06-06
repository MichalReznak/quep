use typed_builder::TypedBuilder;

use crate::args::types::CircuitSchemaType;

#[derive(TypedBuilder, Clone)]
pub struct GenCircuit {
    pub circuit: String,
    pub t: CircuitSchemaType,
}
