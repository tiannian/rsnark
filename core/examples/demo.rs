use rsnark_core::{
    API, Circuit, CircuitBuilder, CircuitDefine, CircuitElement, PrivateCircuitElement,
    PublicCircuitElement, VariableIniter,
};
use ruint::aliases::U256;

pub struct ExampleSubCircuit {
    pub x2: u64,
    x0: u64,
    x1: u64,
}

impl CircuitElement for ExampleSubCircuit {
    type Private = ExampleSubCircuitDefine;
    type Public = ExampleSubCircuitDefine;

    fn create_public(initer: &mut VariableIniter) -> Self::Public {
        ExampleSubCircuitDefine::new(initer)
    }

    fn create_private(initer: &mut VariableIniter) -> Self::Private {
        ExampleSubCircuitDefine::new(initer)
    }

    fn append_public(&self, witness: &mut Vec<U256>) {
        self.x2.append_public(witness);
    }

    fn append_private(&self, witness: &mut Vec<U256>) {
        self.x0.append_private(witness);
        self.x1.append_private(witness);
    }
}

pub struct ExampleSubCircuitDefine {
    x2: PublicCircuitElement<u64>,
    x0: PrivateCircuitElement<u64>,
    x1: PrivateCircuitElement<u64>,
}

impl ExampleSubCircuitDefine {
    pub fn new(initer: &mut VariableIniter) -> Self {
        let x2 = u64::create_public(initer);
        let x0 = u64::create_private(initer);
        let x1 = u64::create_private(initer);

        Self { x2, x0, x1 }
    }
}

impl Circuit for CircuitDefine<ExampleSubCircuit> {
    fn define(&self, api: &mut impl API) {
        let x = api.add(&self.x0, &self.x1, &[]);
        api.assert_is_equal(&x, &self.x2);
    }
}

pub struct ExampleCircuit {
    pub y: u64,
    pub sub_circuit: ExampleSubCircuit,
}

impl CircuitElement for ExampleCircuit {
    type Private = ExampleCircuitDefine;
    type Public = ExampleCircuitDefine;

    fn create_public(initer: &mut VariableIniter) -> Self::Public {
        ExampleCircuitDefine::new(initer)
    }

    fn create_private(initer: &mut VariableIniter) -> Self::Private {
        ExampleCircuitDefine::new(initer)
    }

    fn append_public(&self, witness: &mut Vec<U256>) {
        self.y.append_public(witness);
    }

    fn append_private(&self, witness: &mut Vec<U256>) {
        self.sub_circuit.append_private(witness);
    }
}

pub struct ExampleCircuitDefine {
    y: PublicCircuitElement<u64>,
    sub_circuit: PrivateCircuitElement<ExampleSubCircuit>,
}

impl ExampleCircuitDefine {
    pub fn new(initer: &mut VariableIniter) -> Self {
        let y = u64::create_public(initer);
        let sub_circuit = ExampleSubCircuit::create_private(initer);

        Self { y, sub_circuit }
    }
}

impl Circuit for CircuitDefine<ExampleCircuit> {
    fn define(&self, api: &mut impl API) {
        self.sub_circuit.define(api);

        let inner = api.add(&self.y, &1u64, &[]);
        api.assert_is_equal(&self.y, &inner);
    }
}

fn main() {
    let mut builder = CircuitBuilder::default();
    let circuit = ExampleCircuit::create_private(builder.variable_initer_mut());
    circuit.define(&mut builder);

    let define = builder.build();
    let json = serde_json::to_string_pretty(&define).unwrap();
    println!("{}", json);
}
