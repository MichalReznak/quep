use fehler::throws;

use crate::args::types::*;
use crate::circuit_generators::{
    BaseCircuitGenerator, BasicCircuitGenerator, FsCircuitGenerator, MirrorCircuitGenerator,
    RandMirrorCircuitGenerator, VolumeCircuitGenerator,
};
use crate::ext::{CircuitGeneratorDyn, LangSchemaDyn, OrchestratorDyn, OutputerDyn, QcProviderDyn};
use crate::lang_schemas::{OpenQasmSchema, QiskitSchema};
use crate::orchestrators::{
    LatticeOrchestrator, LinearOrchestrator, SingleOrchestrator, VolumeOrchestrator,
};
use crate::outputers::{SerialOutputer, TextOutputer};
use crate::qc_providers::{IbmqQcProvider, NoisyQcProvider, SimpleQcProvider};
use crate::{CliArgs, Error};
use crate::ext::QcProvider;

/// Args based factory
pub struct Chooser {
    args: CliArgs,
}

impl Chooser {
    pub fn new(args: CliArgs) -> Self {
        Self { args }
    }

    pub fn get_lang_schema(t: CircuitSchemaType) -> LangSchemaDyn {
        use CircuitSchemaType::*;
        match t {
            OpenQasm => LangSchemaDyn::from(OpenQasmSchema::new()),
            Qiskit => LangSchemaDyn::from(QiskitSchema::new()),
        }
    }

    #[throws]
    pub fn get_provider(&self) -> QcProviderDyn {
        use ProviderType::*;
        let prov = match self.args.provider.t {
            Ibmq => QcProviderDyn::from(IbmqQcProvider::new(&self.args.provider)),
            Simple => QcProviderDyn::from(SimpleQcProvider::new(&self.args.provider)),
            Noisy => QcProviderDyn::from(NoisyQcProvider::new(&self.args.provider)),
        };
        prov.check_constraints(&self.args)?;
        prov
    }

    #[throws]
    pub fn get_outputer(&self) -> OutputerDyn {
        use OutputType::*;
        match self.args.output.t {
            Text => OutputerDyn::from(TextOutputer::new(&self.args.output)),
            Serial => OutputerDyn::from(SerialOutputer::new(&self.args.output)),
        }
    }

    #[throws]
    pub fn get_circuit_generator(&self) -> CircuitGeneratorDyn {
        use CircuitType::*;
        match self.args.circuit.t {
            Basic => CircuitGeneratorDyn::from(BasicCircuitGenerator::new(&self.args.circuit)),
            Fs => CircuitGeneratorDyn::from(FsCircuitGenerator::new(&self.args.circuit)),
            Volume => CircuitGeneratorDyn::from(VolumeCircuitGenerator::new(&self.args.circuit)),
            Mirror => CircuitGeneratorDyn::from(MirrorCircuitGenerator::new(&self.args.circuit)),
            RandMirror => {
                CircuitGeneratorDyn::from(RandMirrorCircuitGenerator::new(&self.args.circuit))
            }
            Base => CircuitGeneratorDyn::from(BaseCircuitGenerator::new(&self.args.circuit)),
        }
    }

    #[throws]
    pub fn get_orchestrator(&self) -> OrchestratorDyn {
        use OrchestratorType::*;
        match self.args.orch.t {
            Lattice => OrchestratorDyn::from(LatticeOrchestrator::new(&self.args.orch)),
            Linear => OrchestratorDyn::from(LinearOrchestrator::new(&self.args.orch)),
            Single => OrchestratorDyn::from(SingleOrchestrator::new(&self.args.orch)),
            Volume => OrchestratorDyn::from(VolumeOrchestrator::new(&self.args.orch)),
        }
    }
}
