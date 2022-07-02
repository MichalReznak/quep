use fehler::throws;

use crate::args::types::*;
use crate::circuit_generators::{
    BaseCircuitGenerator, BasicCircuitGenerator, FsCircuitGenerator, PythonCircuitGenerator,
    RandCircuitGenerator, StructCircuitGenerator, VolumeCircuitGenerator,
};
use crate::ext::{
    CircuitGenerator, CircuitGeneratorDyn, LangSchema, LangSchemaDyn, Orchestrator,
    OrchestratorDyn, Outputer, OutputerDyn, QcProvider, QcProviderDyn,
};
use crate::lang_schemas::{OpenQasmSchema, PythonSchema, QiskitSchema};
use crate::orchestrators::{
    LatticeOrchestrator, LinearOrchestrator, SingleOrchestrator, VolumeOrchestrator,
};
use crate::outputers::{PythonOutputer, SerialOutputer, TextOutputer};
use crate::qc_providers::{IbmqQcProvider, NoisyQcProvider, PythonQcProvider, SimpleQcProvider};
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
    pub fn get_lang_schema(&self) -> LangSchemaDyn {
        use CircuitSchemaType::*;
        let res = match self.args.circuit.schema {
            OpenQasm => LangSchemaDyn::from(OpenQasmSchema::from_args()?),
            Qiskit => LangSchemaDyn::from(QiskitSchema::from_args()?),
            Python => LangSchemaDyn::from(PythonSchema::from_args()?),
        };
        res.check_constraints(&self.args)?;
        res
    }

    #[throws]
    pub fn get_provider(&self) -> QcProviderDyn {
        use ProviderType::*;
        let res = match self.args.provider.t {
            Ibmq => QcProviderDyn::from(IbmqQcProvider::from_args(&self.args.provider)?),
            Simple => QcProviderDyn::from(SimpleQcProvider::from_args(&self.args.provider)?),
            Noisy => QcProviderDyn::from(NoisyQcProvider::from_args(&self.args.provider)?),
            Python => QcProviderDyn::from(PythonQcProvider::from_args(&self.args.provider)?),
        };
        res.check_constraints(&self.args)?;
        res
    }

    #[throws]
    pub fn get_outputer(&self) -> OutputerDyn {
        use OutputType::*;
        let res = match self.args.output.t {
            Text => OutputerDyn::from(TextOutputer::from_args(&self.args.output)?),
            Serial => OutputerDyn::from(SerialOutputer::from_args(&self.args.output)?),
            Python => OutputerDyn::from(PythonOutputer::from_args(&self.args.output)?),
        };
        res.check_constraints(&self.args)?;
        res
    }

    #[throws]
    pub fn get_circuit_generator(&self) -> CircuitGeneratorDyn {
        use CircuitType::*;
        let res = match self.args.circuit.t {
            Basic => {
                CircuitGeneratorDyn::from(BasicCircuitGenerator::from_args(&self.args.circuit)?)
            }
            Fs => CircuitGeneratorDyn::from(FsCircuitGenerator::from_args(&self.args.circuit)?),
            Volume => {
                CircuitGeneratorDyn::from(VolumeCircuitGenerator::from_args(&self.args.circuit)?)
            }
            Struct => {
                CircuitGeneratorDyn::from(StructCircuitGenerator::from_args(&self.args.circuit)?)
            }
            Rand => CircuitGeneratorDyn::from(RandCircuitGenerator::from_args(&self.args.circuit)?),
            Base => CircuitGeneratorDyn::from(BaseCircuitGenerator::from_args(&self.args.circuit)?),
            Python => {
                CircuitGeneratorDyn::from(PythonCircuitGenerator::from_args(&self.args.circuit)?)
            }
        };
        res.check_constraints(&self.args)?;
        res
    }

    #[throws]
    pub fn get_orchestrator(&self) -> OrchestratorDyn {
        use OrchestratorType::*;
        let res = match self.args.orch.t {
            Lattice => OrchestratorDyn::from(LatticeOrchestrator::from_args(&self.args.orch)?),
            Linear => OrchestratorDyn::from(LinearOrchestrator::from_args(&self.args.orch)?),
            Single => OrchestratorDyn::from(SingleOrchestrator::from_args(&self.args.orch)?),
            Volume => OrchestratorDyn::from(VolumeOrchestrator::from_args(&self.args.orch)?),
        };
        res.check_constraints(&self.args)?;
        res
    }

    #[throws]
    pub fn get_simulator(&self) -> SimpleQcProvider {
        SimpleQcProvider::from_args(&self.args.provider)?
    }
}
