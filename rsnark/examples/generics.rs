use rsnark::{
    Groth16BN254GnarkProver,
    core::{API, Circuit, CircuitDefine, CircuitWitness, curve::BN254},
};
use rsnark_core::Variable;
use rsnark_provers_core::Backend;
use rsnark_provers_gnark::Groth16Backend;

#[derive(Circuit)]
pub struct TestCircuit<T> {
    a: T,
    b: T,
    pub c: T,
}

impl<T: CircuitWitness> Circuit for CircuitDefine<TestCircuit<T>> {
    fn define(&self, api: &mut impl API) {
        let c = api.add(&self.a, &self.b);
        api.assert_is_equal(&c, &self.c);
    }
}

fn main() {
    // run();
}

// fn run() {
//     let prover = Groth16BN254GnarkProver::new();

//     let circuit_prover = prover.compile_circuit::<TestCircuit<u32>>().unwrap();
//     let (pk, vk) = circuit_prover.setup().unwrap();

//     let circuit_witness = TestCircuit {
//         a: 3,
//         b: 4,
//         c: 7, // 3 + 4 = 7
//     };

//     let proof = circuit_prover.prove(&pk, &circuit_witness).unwrap();

//     let public_witness = circuit_witness.into_public_witness();
//     circuit_prover.verify(&vk, &proof, public_witness).unwrap();
// }
