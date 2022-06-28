//! Add interface description

use async_trait::async_trait;
use pyo3::prelude::*;

use crate::args::CliArgsCircuit;
use crate::ext::types::circuit_generator::GenCircuit;
use crate::ext::CircuitGenerator;
use crate::Error;

#[allow(dead_code)]
pub struct PythonCircuitGenerator {
    args: CliArgsCircuit,
    instance: Option<PyObject>,
}

#[allow(dead_code)]
impl PythonCircuitGenerator {
    pub fn new(args: &CliArgsCircuit) -> Self {
        Self {
            args: args.clone(),
            instance: None,
        }
    }
}

#[async_trait]
impl CircuitGenerator for PythonCircuitGenerator {
    async fn generate(
        &mut self,
        _i: i32,
        _j: i32,
        _iter: i32,
    ) -> Result<Option<GenCircuit>, Error> {
        todo!("Check if is working. Haven't tested it");

        // Python::with_gil(|py| {
        //     if self.instance.is_none() {
        //         // TODO should add some type of path to file
        //         let code =
        //
        // std::fs::read_to_string(&format!("./circuit_generator.py"))?;
        //         let module = PyModule::from_code(py, &code, "", "")?;
        //         let qiskit: Py<PyAny> =
        // module.getattr("CircuitGenerator")?.into();         self.
        // instance = Some(qiskit.call0(py)?);     }
        //
        //     let instance = self.instance.as_ref().unwrap(); // TODO
        //     let res = instance.call_method1(py, "generate", (i, j, iter))?;
        //
        //     if res.is_none(py) {
        //         Ok(None)
        //     }
        //     else {
        //         Ok(Some(GenCircuit::builder()
        //             .t(CircuitSchemaType::from_str("OpenQasm")?)
        //             .circuit(res.to_string())
        //             .build()
        //         ))
        //     }
        // })
    }
}
