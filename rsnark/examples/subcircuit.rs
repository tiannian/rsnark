use rsnark::PlonkBN254GnarkProver;
use rsnark_core::{API, Circuit, CircuitWitness, Witness, circuit};

// Sub-circuit: Adder - computes a + b = sum
#[circuit]
pub struct AdderCircuit {
    a: u32,
    b: u32,
    pub sum: u32,
}

impl Circuit for AdderCircuit {
    fn define(&self, api: &mut impl API) {
        let result = api.add(&self.a, &self.b);
        api.assert_is_equal(&result, &self.sum);
    }
}

// Sub-circuit: Multiplier - computes x * y = product
#[circuit]
pub struct MultiplierCircuit {
    x: u32,
    y: u32,
    pub product: u32,
}

impl Circuit for MultiplierCircuit {
    fn define(&self, api: &mut impl API) {
        let result = api.mul(&self.x, &self.y);
        api.assert_is_equal(&result, &self.product);
    }
}

// Main circuit: Composite circuit containing sub-circuits
#[circuit]
pub struct CompositeCircuit {
    // Embedded sub-circuits
    adder: AdderCircuit,
    pub multiplier: MultiplierCircuit,

    pub final_result: u32,
}

impl Circuit for CompositeCircuit {
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
    let prover = PlonkBN254GnarkProver::new();

    let circuit_prover = prover.compile_circuit::<CompositeCircuit>().unwrap();
    let (pk, vk) = circuit_prover.setup().unwrap();

    // let solidity_contract = vk.export_solidity().unwrap();
    // println!("Solidity contract: {}", solidity_contract);

    let circuit_witness = Witness::<CompositeCircuit> {
        adder: Witness::<AdderCircuit> { a: 1, b: 2, sum: 3 },
        multiplier: Witness::<MultiplierCircuit> {
            x: 2,
            y: 3,
            product: 6,
        },
        final_result: 9,
    };

    let proof = circuit_prover.prove(&pk, &circuit_witness).unwrap();

    let solidity_proof = proof.to_solidity().unwrap();
    println!("Solidity proof: {:#?}", solidity_proof);

    let public_witness = circuit_witness.into_public_witness();
    circuit_prover.verify(&vk, &proof, public_witness).unwrap();
}
