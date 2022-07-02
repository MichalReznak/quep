use std::path::Path;

use fehler::throws;
use pyo3::prelude::*;
use tokio::process::Command;

use crate::Error;

pub struct PyVenv;

impl PyVenv {
    #[throws]
    pub async fn init(dir: &str) -> Self {
        // check if python dir is available
        let venv_dir = format!("{}/.venv", dir);
        let pip = format!("{}/.venv/Scripts/pip", dir);
        let req_str =
            include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "./python/requirements.txt"))
                .to_string();

        // Extend with custom requirements
        let add_reqs = format!("{}/requirements.txt", dir);
        let (is_add_reqs, add_reqs) = if Path::new(&add_reqs).exists() {
            println!("Found additional requirements to the venv...");
            (true, std::fs::read_to_string(&add_reqs)?)
        }
        else {
            (false, "".to_string())
        };

        let mut reqs = vec!["install"];
        reqs.extend(req_str.split("\r\n").filter(|e| !e.is_empty()));
        reqs.extend(add_reqs.split("\r\n").filter(|e| !e.is_empty()));

        if !Path::new(&venv_dir).exists() {
            // install .venv
            println!("Installing virtualenv...");
            Command::new("python").args(["-m", "venv", &venv_dir]).spawn()?.wait().await?;
        }

        // Check if qiskit exists
        if is_add_reqs || !Path::new(&format!("{}/cmake", &venv_dir)).exists() {
            // run in venv pip install
            println!("Installing python dependencies...");
            Command::new(&pip).args(&reqs).spawn()?.wait().await?;
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
