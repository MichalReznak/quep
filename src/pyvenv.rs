use std::path::Path;

use fehler::throws;
use log::info;
use pyo3::prelude::*;
use tokio::process::Command;

use crate::Error;

pub struct PyVenv;

impl PyVenv {
    #[throws]
    pub async fn init(dir: &str) -> Self {
        // check if python dir is available
        println!("{}", dir);
        let venv_dir = format!("{}/.venv", dir);
        let pip = format!("{}/.venv/Scripts/pip", dir);
        let req = format!("{}/requirements.txt", dir);

        if !Path::new(&venv_dir).exists() {
            // install .venv
            info!("Installing virtualenv...");
            Command::new("python").args(["-m", "venv", &venv_dir]).spawn()?.wait().await?;
        }

        // Check if qiskit exists
        if !Path::new(&format!("{}/cmake", &venv_dir)).exists() {
            // run in venv pip install
            info!("Installing qiskit...");
            Command::new(&pip).args(["install", "-r", &req]).spawn()?.wait().await?;
        }

        // set correct paths
        Python::with_gil(|py| -> Result<_, Error> {
            Ok(Python::run(
                py,
                &unindent::unindent(&format!(
                    r#"
                        import sys
                        sys.path.append('{}/.venv/lib/site-packages')
                        sys.path.append('{}/.venv')
                    "#,
                    &dir, &dir
                )),
                None,
                None,
            )?)
        })?;

        Self
    }
}