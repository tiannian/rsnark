use rsnark::core::{API, Circuit, CircuitWitness, circuit, curve::BN254};
use rsnark_core::Witness;
use rsnark_provers_core::{Backend, Prover};
use rsnark_provers_gnark::Groth16Backend;

#[circuit]
pub struct TestCircuit {
    a: u32,
    b: u32,
    pub c: u32,
}

impl Circuit for TestCircuit {
    fn define(&self, api: &mut impl API) {
        let c = api.add(&self.a, &self.b);

        let d = api.add(&c, &10);

        api.assert_is_equal(&d, &self.c);
    }
}

fn main() {
    run::<Groth16Backend<BN254>>();
    // run::<Groth16Backend<BLS12_377>>();
    // run::<Groth16Backend<BLS12_381>>();
    // run::<Groth16Backend<BLS24_315>>();
    // run::<Groth16Backend<BLS24_317>>();
    // run::<Groth16Backend<BW6_761>>();
    // run::<Groth16Backend<BW6_633>>();

    // run::<PlonkBackend<BN254>>();
    // run::<PlonkBackend<BLS12_377>>();
    // run::<PlonkBackend<BLS12_381>>();
    // run::<PlonkBackend<BLS24_315>>();
    // run::<PlonkBackend<BLS24_317>>();
    // run::<PlonkBackend<BW6_761>>();
    // run::<PlonkBackend<BW6_633>>();
}

fn run<B: Backend>() {
    let prover = Prover::<B>::new();

    let circuit_prover = prover.compile_circuit::<TestCircuit>().unwrap();
    let (pk, vk) = circuit_prover.setup().unwrap();

    let circuit_witness = Witness::<TestCircuit> {
        a: 3,
        b: 4,
        c: 17, // 3 + 4 = 7
    };

    let proof = circuit_prover.prove(&pk, &circuit_witness).unwrap();

    let public_witness = circuit_witness.into_public_witness();
    circuit_prover.verify(&vk, &proof, public_witness).unwrap();
}
