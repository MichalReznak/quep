use typed_builder::TypedBuilder;

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

    // TODO other types of gates
    Barrier,
}

impl LangGateType {
    pub fn inverse(&self) -> Self {
        use LangGateType::*;
        match self {
            Id => Id,
            X => X,
            Y => Y,
            Z => Z,
            H => H,
            S => Sdg,
            Sdg => S,
            Cx => Cx,
            Cz => Cz,
            Swap => Swap,
            Barrier => Barrier,
        }
    }
}

#[derive(TypedBuilder, Clone)]
pub struct LangGate {
    pub t: LangGateType,

    pub i: i32,

    #[builder(default, setter(strip_option))]
    pub other: Option<i32>,
}

impl LangGate {
    pub fn inverse(&self) -> Self {
        if let Some(o) = self.other {
            Self::builder()
                .t(self.t.inverse())
                .i(self.i)
                .other(o)
                .build()
        }else {
            Self::builder()
                .t(self.t.inverse())
                .i(self.i)
                .build()
        }
    }
}

// TODO rename?
#[derive(TypedBuilder, Clone)]
pub struct LangCircuit {
    pub width: i32,
    pub gates: Vec<LangGate>,
    pub inv_gates: Vec<LangGate>,
}
