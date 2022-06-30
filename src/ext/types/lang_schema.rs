use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;
use pyo3::pyclass;

#[pyclass]
#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum LangGateType {
    Id,
    X,
    Y,
    Z,
    H,
    S,
    Sdg,
    Cx,
    Cz,
    Swap,

    // TODO other types of gates
    Barrier,
    Dummy,
}

impl LangGateType {
    pub fn inverse(&self) -> Self {
        use LangGateType::*;
        match self {
            Id => Id,
            X => X,
            Y => Y,
            Z => Z,
            H => H,
            S => Sdg,
            Sdg => S,
            Cx => Cx,
            Cz => Cz,
            Swap => Swap,
            Barrier => Barrier, // TODO support with multiple definitions -1 is with no index
            Dummy => Dummy,
        }
    }
}

#[pyclass]
#[derive(TypedBuilder, Clone, Debug, Serialize, Deserialize)]
pub struct LangGate {
    #[pyo3(get, set)]
    pub t: LangGateType,

    #[pyo3(get, set)]
    pub i: i32,

    #[pyo3(get, set)]
    #[builder(default, setter(strip_option))]
    pub other: Option<i32>,
}

impl LangGate {
    pub fn inverse(&self) -> Self {
        if let Some(o) = self.other {
            Self::builder().t(self.t.inverse()).i(self.i).other(o).build()
        }
        else {
            Self::builder().t(self.t.inverse()).i(self.i).build()
        }
    }
}

#[pyclass]
#[derive(TypedBuilder, Clone, Serialize, Deserialize)]
pub struct LangCircuit {
    #[pyo3(get, set)]
    pub width: i32,

    #[pyo3(get, set)]
    pub gates: Vec<LangGate>,
}
