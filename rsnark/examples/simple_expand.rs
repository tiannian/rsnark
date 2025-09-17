use rsnark::core::{
    API, Circuit, CircuitDefine, CircuitWitness,
    curve::{BLS12_377, BLS12_381, BLS24_315, BLS24_317, BN254, BW6_633, BW6_761},
};
use rsnark_provers_core::{Backend, Prover};
use rsnark_provers_gnark::{Groth16Backend, PlonkBackend};
pub struct TestCircuit {
    a: u32,
    b: u32,
    pub c: u32,
}
mod __rsnark_generated_testcircuit {
    use super::*;
    use ::rsnark_core::{BigInt, CircuitPublicWitness, CircuitWitness, VariableIniter};
    impl CircuitWitness for TestCircuit {
        type CircuitElement = TestCircuitCircuitDefine;
        type PublicWitness = TestCircuitPublicWitness;
        fn create_public(initer: &mut VariableIniter, is_private: bool) -> Self::CircuitElement {
            TestCircuitCircuitDefine::new(initer, is_private)
        }
        fn create_private(initer: &mut VariableIniter) -> Self::CircuitElement {
            TestCircuitCircuitDefine::new(initer, true)
        }
        fn append_witness(
            &self,
            public: &mut Vec<BigInt>,
            private: &mut Vec<BigInt>,
            _is_private: bool,
        ) {
            self.a.append_witness(public, private, true);
            self.b.append_witness(public, private, true);
            self.c.append_witness(public, private, false || _is_private);
        }
        fn into_public_witness(self) -> Self::PublicWitness {
            TestCircuitPublicWitness {
                c: self.c.into_public_witness(),
            }
        }
    }
    #[doc(hidden)]
    pub struct TestCircuitCircuitDefine {
        pub a: <u32 as ::rsnark_core::CircuitWitness>::CircuitElement,
        pub b: <u32 as ::rsnark_core::CircuitWitness>::CircuitElement,
        pub c: <u32 as ::rsnark_core::CircuitWitness>::CircuitElement,
    }
    impl TestCircuitCircuitDefine {
        fn new(initer: &mut VariableIniter, is_private: bool) -> Self {
            let a = u32::create_private(initer);
            let b = u32::create_private(initer);
            let c = u32::create_public(initer, is_private);
            Self { a, b, c }
        }
    }
    impl CircuitPublicWitness for TestCircuit {
        fn append_public_witness(&self, witness: &mut Vec<BigInt>, _is_private: bool) {
            self.c.append_public_witness(witness, false);
        }
    }
    #[doc(hidden)]
    pub struct TestCircuitPublicWitness {
        pub c: ::rsnark_core::PublicWitness<u32>,
    }
    impl CircuitPublicWitness for TestCircuitPublicWitness {
        fn append_public_witness(&self, witness: &mut Vec<BigInt>, _is_private: bool) {
            self.c.append_public_witness(witness, false);
        }
    }
}
impl Circuit for CircuitDefine<TestCircuit> {
    fn define(&self, api: &mut impl API) {
        let c = api.add(&self.a, &self.b);
        api.assert_is_equal(&c, &self.c);
    }
}
fn main() {
    run::<Groth16Backend<BN254>>();
    run::<Groth16Backend<BLS12_377>>();
    run::<Groth16Backend<BLS12_381>>();
    run::<Groth16Backend<BLS24_315>>();
    run::<Groth16Backend<BLS24_317>>();
    run::<Groth16Backend<BW6_761>>();
    run::<Groth16Backend<BW6_633>>();
    run::<PlonkBackend<BN254>>();
    run::<PlonkBackend<BLS12_377>>();
    run::<PlonkBackend<BLS12_381>>();
    run::<PlonkBackend<BLS24_315>>();
    run::<PlonkBackend<BLS24_317>>();
    run::<PlonkBackend<BW6_761>>();
    run::<PlonkBackend<BW6_633>>();
}
fn run<B: Backend>() {
    let prover = Prover::<B>::new();
    let circuit_prover = prover.compile_circuit::<TestCircuit>().unwrap();
    let (pk, vk) = circuit_prover.setup().unwrap();
    let circuit_witness = TestCircuit { a: 3, b: 4, c: 7 };
    let proof = circuit_prover.prove(&pk, &circuit_witness).unwrap();
    let public_witness = circuit_witness.into_public_witness();
    circuit_prover.verify(&vk, &proof, public_witness).unwrap();
}
