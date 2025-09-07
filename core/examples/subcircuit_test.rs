use rsnark_core::{API, Circuit, CircuitBuilder, CircuitDefine, CircuitElement};

// Sub-circuit: Adder - computes a + b = sum
#[derive(Circuit)]
pub struct AdderCircuit {
    a: u32,
    b: u32,
    pub sum: u32,
}

impl Circuit for CircuitDefine<AdderCircuit> {
    fn define(&self, api: &mut impl API) {
        let result = api.add(&self.a, &self.b);
        api.assert_is_equal(&result, &self.sum);
    }
}

// Sub-circuit: Multiplier - computes x * y = product
#[derive(Circuit)]
pub struct MultiplierCircuit {
    x: u32,
    y: u32,
    pub product: u32,
}

impl Circuit for CircuitDefine<MultiplierCircuit> {
    fn define(&self, api: &mut impl API) {
        let result = api.mul(&self.x, &self.y);
        api.assert_is_equal(&result, &self.product);
    }
}

// Main circuit: Composite circuit containing sub-circuits
#[derive(Circuit)]
pub struct CompositeCircuit {
    // Embedded sub-circuits
    adder: AdderCircuit,
    multiplier: MultiplierCircuit,

    pub final_result: u32,
}

impl Circuit for CircuitDefine<CompositeCircuit> {
    fn define(&self, api: &mut impl API) {
        // 1. Execute adder sub-circuit
        self.adder.define(api);

        // 2. Execute multiplier sub-circuit
        self.multiplier.define(api);

        // 3. Main circuit logic: add sub-circuit results
        let final_sum = api.add(&self.adder.sum, &self.multiplier.product);
        api.assert_is_equal(&final_sum, &self.final_result);
    }
}

fn main() {
    println!("Sub-circuit test compilation successful!");
    println!();
    println!("Circuit architecture:");
    println!("  AdderCircuit: a + b = sum");
    println!("  MultiplierCircuit: x * y = product");
    println!("  CompositeCircuit: (adder.sum + multiplier.product) = final_result");
    println!();

    // Build and execute the circuit
    let mut builder = CircuitBuilder::default();
    let circuit = CompositeCircuit::create_private(builder.variable_initer_mut());
    circuit.define(&mut builder);

    let define = builder.build();
    let json = serde_json::to_string_pretty(&define).unwrap();
    println!("Generated circuit definition:");
    println!("{}", json);
}
