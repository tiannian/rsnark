use rsnark::core::{API, Circuit, CircuitDefine, CircuitWitness};
use rsnark::provers::core::{Curve, Prover};
use rsnark::provers::gnark::Groth16Backend;

#[derive(Circuit)]
pub struct TestCircuit {
    a: u32,
    b: u32,
    pub c: u32,
}

impl Circuit for CircuitDefine<TestCircuit> {
    fn define(&self, api: &mut impl API) {
        let c = api.add(&self.a, &self.b);
        api.assert_is_equal(&c, &self.c);
    }
}

fn main() {
    let prover: Prover<Groth16Backend> = Prover::new(Curve::BN254);

    let circuit_prover = prover.compile_circuit::<TestCircuit>().unwrap();

    let (pk, vk) = circuit_prover.setup().unwrap();

    let circuit_witness = TestCircuit {
        a: 3,
        b: 4,
        c: 7, // 3 + 4 = 7
    };

    let proof = circuit_prover.prove(&pk, &circuit_witness).unwrap();

    let public_witness = circuit_witness.into_public_witness();
    circuit_prover.verify(&vk, &proof, public_witness).unwrap();
}
