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
}

#[derive(TypedBuilder)]
pub struct LangGate {
    pub t: LangGateType,

    pub i: i32,

    #[builder(default, setter(strip_option))]
    pub other: Option<i32>,
}

// TODO rename?
#[derive(TypedBuilder)]
pub struct LangCircuit {
    pub width: i32,
    pub gates: Vec<LangGate>,
    pub inv_gates: Vec<LangGate>,
}
