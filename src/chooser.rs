use std::intrinsics::unreachable;
use std::str::FromStr;

use fehler::throws;

use crate::args::types::*;
use crate::circuit_generators::{BasicCircuitGenerator, FsCircuitGenerator};
use crate::outputers::TextOutputer;
use crate::traits::QcProviderDyn;

#[cfg(feature = "qiskit")]
use crate::qc_providers::{QiskitQcProvider, NoisyQcProvider};

use crate::qc_providers::{IbmqQcProvider};
use crate::traits::{CircuitGeneratorDyn, OutputerDyn};
use crate::{Error, ARGS};

// Args based factory
pub struct Chooser;

impl Chooser {
    #[cfg(feature = "qiskit")]
    #[throws]
    pub fn get_provider() -> QcProviderDyn {
        match ProviderType::from_str(&ARGS.provider)? {
            ProviderType::Ibmq => QcProviderDyn::from(IbmqQcProvider::new()),
            ProviderType::Qiskit => QcProviderDyn::from(QiskitQcProvider::new()),
            ProviderType::Noisy => QcProviderDyn::from(NoisyQcProvider::new()),
        }
    }

    #[cfg(not(feature = "qiskit"))]
    #[throws]
    pub fn get_provider() -> QcProviderDyn {
        match ProviderType::from_str(&ARGS.provider)? {
            ProviderType::Ibmq => QcProviderDyn::from(IbmqQcProvider::new()),
            ProviderType::Qiskit => unreachable!(),
            ProviderType::Noisy => unreachable!(),
        }
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
        match CircuitType::from_str(&ARGS.circuit)? {
            CircuitType::Basic => CircuitGeneratorDyn::from(BasicCircuitGenerator::new()),
            CircuitType::Fs => CircuitGeneratorDyn::from(FsCircuitGenerator::new()),
        }
    }
}
