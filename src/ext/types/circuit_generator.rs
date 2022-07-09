use pyo3::pyclass;
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

use crate::args::types::LangSchemaType;

#[pyclass]
#[derive(TypedBuilder, Clone, Serialize, Deserialize)]
pub struct GenCircuit {
    #[pyo3(get, set)]
    pub circuit: String,

    #[pyo3(get, set)]
    pub t: LangSchemaType,
}
