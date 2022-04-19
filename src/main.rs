use fehler::throws;
use pyo3::ffi::{Py_SetPath, Py_GetPath, Py_SetPythonHome, Py_Initialize, PySys_SetPath};
use wchar::{wch, wchz};


#[throws(anyhow::Error)]
#[tokio::main]
async fn main() {
    env_logger::init();

    quep::Quep::new().await.run().await?; // .run()
}
