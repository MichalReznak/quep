use pyo3::pyclass;
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

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
            Barrier => Barrier,
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
    pub creg: i32,

    #[pyo3(get, set)]
    pub qreg: i32,

    #[pyo3(get, set)]
    pub gates: Vec<LangGate>,
}

#[pyclass]
#[derive(TypedBuilder, Clone, Serialize, Deserialize)]
pub struct ParsedLangCircuit {
    #[builder(default=i32::MAX)]
    #[pyo3(get, set)]
    pub creg: i32,

    #[builder(default=i32::MAX)]
    #[pyo3(get, set)]
    pub qreg: i32,

    #[pyo3(get, set)]
    pub gates: Vec<LangGate>,
}
