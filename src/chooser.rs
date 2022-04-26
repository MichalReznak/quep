use fehler::throws;

use crate::args::types::*;
use crate::circuit_generators::{
    BasicCircuitGenerator, FsCircuitGenerator, MirrorCircuitGenerator, RandMirrorCircuitGenerator,
    VolumeCircuitGenerator,
};
use crate::orchestrators::{
    LatticeOrchestrator, LinearOrchestrator, SingleOrchestrator, VolumeOrchestrator,
};
use crate::outputers::{SerialOutputer, TextOutputer};
use crate::qc_providers::IbmqQcProvider;
#[cfg(feature = "qiskit")]
use crate::qc_providers::{NoisyQcProvider, QiskitQcProvider};
use crate::traits::{CircuitGeneratorDyn, OrchestratorDyn, OutputerDyn, QcProviderDyn};
use crate::{Error, ARGS};

/// Args based factory
pub struct Chooser;

impl Chooser {
    #[cfg(feature = "qiskit")]
    #[throws]
    pub fn get_provider() -> QcProviderDyn {
        use ProviderType::*;
        match ARGS.provider {
            Ibmq => QcProviderDyn::from(IbmqQcProvider::new()),
            Qiskit => QcProviderDyn::from(QiskitQcProvider::new()),
            Noisy => QcProviderDyn::from(NoisyQcProvider::new()),
        }
    }

    #[cfg(not(feature = "qiskit"))]
    #[throws]
    pub fn get_provider() -> QcProviderDyn {
        use ProviderType::*;
        match ARGS.provider {
            Ibmq => QcProviderDyn::from(IbmqQcProvider::new()),
            _ => unreachable!(),
        }
    }

    #[throws]
    pub fn get_outputer() -> OutputerDyn {
        use OutputType::*;
        match ARGS.output {
            Text => OutputerDyn::from(TextOutputer::new()),
            Serial => OutputerDyn::from(SerialOutputer::new()),
        }
    }

    #[throws]
    pub fn get_orchestrator() -> OrchestratorDyn {
        use OrchestratorType::*;
        match ARGS.orch {
            Lattice => OrchestratorDyn::from(LatticeOrchestrator::new()),
            Linear => OrchestratorDyn::from(LinearOrchestrator::new()),
            Single => OrchestratorDyn::from(SingleOrchestrator::new()),
            Volume => OrchestratorDyn::from(VolumeOrchestrator::new()),
        }
    }

    #[throws]
    pub fn get_circuit_generator() -> CircuitGeneratorDyn {
        use CircuitType::*;
        match ARGS.circuit {
            Basic => CircuitGeneratorDyn::from(BasicCircuitGenerator::new()),
            Fs => CircuitGeneratorDyn::from(FsCircuitGenerator::new()),
            Volume => CircuitGeneratorDyn::from(VolumeCircuitGenerator::new()),
            Mirror => CircuitGeneratorDyn::from(MirrorCircuitGenerator::new()),
            RandMirror => CircuitGeneratorDyn::from(RandMirrorCircuitGenerator::new()),
        }
    }
}
