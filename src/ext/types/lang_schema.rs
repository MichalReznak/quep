use derive_more::Constructor;
use fehler::throws;
use typed_builder::TypedBuilder;

use crate::Error;

// TODO better?
#[derive(Copy, Clone)]
pub enum LangGateType {
    Id,
    X,
    Y,
    Z,
    H,
    S,
    Sdg,
    Cx,
    Cz,
    Swap,
}

#[derive(TypedBuilder)]
pub struct LangGate {
    t: LangGateType,

    i: i32,

    #[builder(default, setter(strip_option))]
    other: Option<i32>,
}

impl ToString for LangGate {
    // TODO remove fixed q
    // TODO should not implement openqasm directly
    fn to_string(&self) -> String {
        match self.t {
            LangGateType::Id => format!("id q[{}];", self.i),
            LangGateType::X => format!("x q[{}];", self.i),
            LangGateType::Y => format!("y q[{}];", self.i),
            LangGateType::Z => format!("z q[{}];", self.i),
            LangGateType::H => format!("h q[{}];", self.i),
            LangGateType::S => format!("s q[{}];", self.i),
            LangGateType::Sdg => format!("sdg q[{}];", self.i),
            LangGateType::Cx => format!("cx q[{}], q[{}];", self.i, self.other.unwrap()),
            LangGateType::Cz => format!("cz q[{}], q[{}];", self.i, self.other.unwrap()),
            LangGateType::Swap => format!("swap q[{}], q[{}];", self.i, self.other.unwrap()),
        }
    }
}

// TODO rename?
#[derive(TypedBuilder)]
pub struct LangCircuit {
    pub width: i32,
    pub gates: Vec<LangGate>,
    pub inv_gates: Vec<LangGate>,
}

impl LangCircuit {
    #[throws]
    pub fn get_parsed() -> String {
        todo!()
    }
}
