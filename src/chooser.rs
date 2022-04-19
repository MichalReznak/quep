use fehler::throws;

use crate::args::types::*;
use crate::circuit_generators::{
    BasicCircuitGenerator, FsCircuitGenerator, MirrorCircuitGenerator, VolumeCircuitGenerator,
};
use crate::outputers::TextOutputer;
use crate::qc_providers::IbmqQcProvider;
#[cfg(feature = "qiskit")]
use crate::qc_providers::{NoisyQcProvider, QiskitQcProvider};
use crate::traits::{CircuitGeneratorDyn, OutputerDyn, QcProviderDyn};
use crate::{Error, ARGS};

// Args based factory
pub struct Chooser;

impl Chooser {
    #[cfg(feature = "qiskit")]
    #[throws]
    pub fn get_provider() -> QcProviderDyn {
        match ARGS.provider {
            ProviderType::Ibmq => QcProviderDyn::from(IbmqQcProvider::new()),
            ProviderType::Qiskit => QcProviderDyn::from(QiskitQcProvider::new()),
            ProviderType::Noisy => QcProviderDyn::from(NoisyQcProvider::new()),
        }
    }

    #[cfg(not(feature = "qiskit"))]
    #[throws]
    pub fn get_provider() -> QcProviderDyn {
        match ARGS.provider {
            ProviderType::Ibmq => QcProviderDyn::from(IbmqQcProvider::new()),
            ProviderType::Qiskit => unreachable!(),
            ProviderType::Noisy => unreachable!(),
        }
    }

    #[throws]
    pub fn get_outputer() -> OutputerDyn {
        let res = match ARGS.output {
            OutputType::Text => TextOutputer::new(),
        };

        OutputerDyn::from(res)
    }

    #[throws]
    pub fn get_circuit_generator() -> CircuitGeneratorDyn {
        match ARGS.circuit {
            CircuitType::Basic => CircuitGeneratorDyn::from(BasicCircuitGenerator::new()),
            CircuitType::Fs => CircuitGeneratorDyn::from(FsCircuitGenerator::new()),
            CircuitType::Volume => CircuitGeneratorDyn::from(VolumeCircuitGenerator::new()),
            CircuitType::Mirror => CircuitGeneratorDyn::from(MirrorCircuitGenerator::new()),
        }
    }
}
