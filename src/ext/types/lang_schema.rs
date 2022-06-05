
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
        use LangGateType::*;
        match self.t {
            Id => format!("id q[{}];", self.i),
            X => format!("x q[{}];", self.i),
            Y => format!("y q[{}];", self.i),
            Z => format!("z q[{}];", self.i),
            H => format!("h q[{}];", self.i),
            S => format!("s q[{}];", self.i),
            Sdg => format!("sdg q[{}];", self.i),
            Cx => format!("cx q[{}], q[{}];", self.i, self.other.unwrap()),
            Cz => format!("cz q[{}], q[{}];", self.i, self.other.unwrap()),
            Swap => format!("swap q[{}], q[{}];", self.i, self.other.unwrap()),
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
