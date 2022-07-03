use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

use crate::args::types::LangSchemaType;

#[derive(TypedBuilder, Clone, Serialize, Deserialize)]
pub struct GenCircuit {
    pub circuit: String,
    pub t: LangSchemaType,
}
