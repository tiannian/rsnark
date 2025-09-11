use rsnark::{
    Groth16BN254GnarkProver,
    core::{API, Circuit, CircuitDefine, CircuitWitness},
};
pub struct TestCircuit {
    a: u32,
    b: u32,
    pub c: u32,
}
mod __rsnark_generated_testcircuit {
    use super::*;
    use ::rsnark_core::{CircuitPublicWitness, CircuitWitness, U256, VariableIniter};
    impl CircuitWitness for TestCircuit {
        type PrivateElement = TestCircuitCircuitDefine;
        type PublicElement = TestCircuitCircuitDefine;
        type PublicWitness = TestCircuitPublicWitness;
        fn create_public(initer: &mut VariableIniter) -> Self::PublicElement {
            TestCircuitCircuitDefine::new(initer)
        }
        fn create_private(initer: &mut VariableIniter) -> Self::PrivateElement {
            TestCircuitCircuitDefine::new(initer)
        }
        fn append_witness(
            &self,
            public: &mut Vec<U256>,
            private: &mut Vec<U256>,
            _is_private: bool,
        ) {
            self.a.append_witness(public, private, true);
            self.b.append_witness(public, private, true);
            self.c.append_witness(public, private, false);
        }
        fn into_public_witness(self) -> Self::PublicWitness {
            TestCircuitPublicWitness { c: self.c }
        }
    }
    #[doc(hidden)]
    pub struct TestCircuitCircuitDefine {
        pub a: ::rsnark_core::PrivateCircuitElement<u32>,
        pub b: ::rsnark_core::PrivateCircuitElement<u32>,
        pub c: ::rsnark_core::PublicCircuitElement<u32>,
    }
    impl TestCircuitCircuitDefine {
        fn new(initer: &mut VariableIniter) -> Self {
            let a = <u32 as ::rsnark_core::CircuitWitness>::create_private(initer);
            let b = <u32 as ::rsnark_core::CircuitWitness>::create_private(initer);
            let c = <u32 as ::rsnark_core::CircuitWitness>::create_public(initer);
            Self { a, b, c }
        }
    }
    impl CircuitPublicWitness for TestCircuit {
        fn append_public_witness(&self, witness: &mut Vec<U256>, _is_private: bool) {
            self.a.append_public_witness(witness, true);
            self.b.append_public_witness(witness, true);
            self.c.append_public_witness(witness, false);
        }
    }
    #[doc(hidden)]
    pub struct TestCircuitPublicWitness {
        pub c: u32,
    }
    impl CircuitPublicWitness for TestCircuitPublicWitness {
        fn append_public_witness(&self, witness: &mut Vec<U256>, _is_private: bool) {
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
    let prover = Groth16BN254GnarkProver::new();
    let circuit_prover = prover.compile_circuit::<TestCircuit>().unwrap();
    let (pk, vk) = circuit_prover.setup().unwrap();
    let circuit_witness = TestCircuit { a: 3, b: 4, c: 7 };
    let proof = circuit_prover.prove(&pk, &circuit_witness).unwrap();
    let public_witness = circuit_witness.into_public_witness();
    circuit_prover.verify(&vk, &proof, public_witness).unwrap();
}
