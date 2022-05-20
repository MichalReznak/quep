use fehler::throws;

use crate::args::types::*;
use crate::circuit_generators::{
    BaseCircuitGenerator, BasicCircuitGenerator, FsCircuitGenerator, MirrorCircuitGenerator,
    RandMirrorCircuitGenerator, VolumeCircuitGenerator,
};
use crate::orchestrators::{
    LatticeOrchestrator, LinearOrchestrator, SingleOrchestrator, VolumeOrchestrator,
};
use crate::outputers::{SerialOutputer, TextOutputer};
use crate::qc_providers::{IbmqQcProvider, NoisyQcProvider, SimpleQcProvider};
use crate::traits::{CircuitGeneratorDyn, OrchestratorDyn, OutputerDyn, QcProviderDyn};
use crate::{CliArgs, Error};

/// Args based factory
pub struct Chooser {
    args: CliArgs,
}

impl Chooser {
    pub fn new(args: CliArgs) -> Self {
        Self { args }
    }

    #[throws]
    pub fn get_provider(&self) -> QcProviderDyn {
        use ProviderType::*;
        match self.args.provider {
            Ibmq => QcProviderDyn::from(IbmqQcProvider::new(&self.args.python_dir)),
            Simple => QcProviderDyn::from(SimpleQcProvider::new(&self.args.python_dir)),
            Noisy => QcProviderDyn::from(NoisyQcProvider::new(&self.args.python_dir)),
        }
    }

    #[throws]
    pub fn get_outputer(&self) -> OutputerDyn {
        use OutputType::*;
        match self.args.output {
            Text => OutputerDyn::from(TextOutputer::new()),
            Serial => OutputerDyn::from(SerialOutputer::new(self.args.output_ser)),
        }
    }

    #[throws]
    pub fn get_orchestrator(&self) -> OrchestratorDyn {
        use OrchestratorType::*;
        match self.args.orch {
            Lattice => OrchestratorDyn::from(LatticeOrchestrator::new()),
            Linear => OrchestratorDyn::from(LinearOrchestrator::new()),
            Single => OrchestratorDyn::from(SingleOrchestrator::new()),
            Volume => OrchestratorDyn::from(VolumeOrchestrator::new()),
        }
    }

    #[throws]
    pub fn get_circuit_generator(&self) -> CircuitGeneratorDyn {
        use CircuitType::*;
        match self.args.circuit {
            Basic => CircuitGeneratorDyn::from(BasicCircuitGenerator::new()),
            Fs => CircuitGeneratorDyn::from(FsCircuitGenerator::new()),
            Volume => CircuitGeneratorDyn::from(VolumeCircuitGenerator::new()),
            Mirror => CircuitGeneratorDyn::from(MirrorCircuitGenerator::new()),
            RandMirror => CircuitGeneratorDyn::from(RandMirrorCircuitGenerator::new()),
            Base => CircuitGeneratorDyn::from(BaseCircuitGenerator::new()),
        }
    }
}
