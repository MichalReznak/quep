use fehler::throws;

use crate::circuit_generators::BasicCircuitGenerator;
use crate::outputers::TextOutputer;
use crate::qc_providers::IbmqQcProvider;
use crate::traits::{CircuitGeneratorDyn, OutputerDyn, QcProviderDyn};
use crate::{Error, ARGS};
use crate::args::types::*;
use std::str::FromStr;

// Args based factory
pub struct Chooser;

impl Chooser {
    #[throws]
    pub fn get_provider() -> QcProviderDyn {
        let res = match ProviderType::from_str(&ARGS.provider)? {
            ProviderType::Ibmq => IbmqQcProvider::new(),
        };

        QcProviderDyn::from(res)
    }

    #[throws]
    pub fn get_outputer() -> OutputerDyn {
        let res = match OutputType::from_str(&ARGS.output)? {
            OutputType::Text => TextOutputer::new(),
        };

        OutputerDyn::from(res)
    }

    #[throws]
    pub fn get_circuit_generator() -> CircuitGeneratorDyn {
        let res = match CircuitType::from_str(&ARGS.circuit)? {
            CircuitType::Basic => BasicCircuitGenerator::new(),
        };

        CircuitGeneratorDyn::from(res)
    }
}
