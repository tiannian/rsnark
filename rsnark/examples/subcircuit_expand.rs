use rsnark::Groth16BN254GnarkProver;
use rsnark_core::{API, Circuit, CircuitDefine, CircuitWitness};
pub struct AdderCircuit {
    a: u32,
    b: u32,
    pub sum: u32,
}
mod __rsnark_generated_addercircuit {
    use super::*;
    use ::rsnark_core::{CircuitPublicWitness, CircuitWitness, U256, VariableIniter};
    impl CircuitWitness for AdderCircuit {
        type PrivateElement = AdderCircuitCircuitDefine;
        type PublicElement = AdderCircuitCircuitDefine;
        type PublicWitness = AdderCircuitPublicWitness;
        fn create_public(initer: &mut VariableIniter) -> Self::PublicElement {
            AdderCircuitCircuitDefine::new(initer)
        }
        fn create_private(initer: &mut VariableIniter) -> Self::PrivateElement {
            AdderCircuitCircuitDefine::new(initer)
        }
        fn append_witness(
            &self,
            public: &mut Vec<U256>,
            private: &mut Vec<U256>,
            _is_private: bool,
        ) {
            self.a.append_witness(public, private, true);
            self.b.append_witness(public, private, true);
            self.sum.append_witness(public, private, false);
        }
        fn into_public_witness(self) -> Self::PublicWitness {
            AdderCircuitPublicWitness { sum: self.sum }
        }
    }
    #[doc(hidden)]
    pub struct AdderCircuitCircuitDefine {
        pub a: ::rsnark_core::PrivateCircuitElement<u32>,
        pub b: ::rsnark_core::PrivateCircuitElement<u32>,
        pub sum: ::rsnark_core::PublicCircuitElement<u32>,
    }
    impl AdderCircuitCircuitDefine {
        fn new(initer: &mut VariableIniter) -> Self {
            let a = <u32 as ::rsnark_core::CircuitWitness>::create_private(initer);
            let b = <u32 as ::rsnark_core::CircuitWitness>::create_private(initer);
            let sum = <u32 as ::rsnark_core::CircuitWitness>::create_public(initer);
            Self { a, b, sum }
        }
    }
    impl CircuitPublicWitness for AdderCircuit {
        fn append_public_witness(&self, witness: &mut Vec<U256>, _is_private: bool) {
            self.a.append_public_witness(witness, true);
            self.b.append_public_witness(witness, true);
            self.sum.append_public_witness(witness, false);
        }
    }
    #[doc(hidden)]
    pub struct AdderCircuitPublicWitness {
        pub sum: u32,
    }
    impl CircuitPublicWitness for AdderCircuitPublicWitness {
        fn append_public_witness(&self, witness: &mut Vec<U256>, _is_private: bool) {
            self.sum.append_public_witness(witness, false);
        }
    }
}
impl Circuit for CircuitDefine<AdderCircuit> {
    fn define(&self, api: &mut impl API) {
        let result = api.add(&self.a, &self.b);
        api.assert_is_equal(&result, &self.sum);
    }
}
pub struct MultiplierCircuit {
    x: u32,
    y: u32,
    pub product: u32,
}
mod __rsnark_generated_multipliercircuit {
    use super::*;
    use ::rsnark_core::{CircuitPublicWitness, CircuitWitness, U256, VariableIniter};
    impl CircuitWitness for MultiplierCircuit {
        type PrivateElement = MultiplierCircuitCircuitDefine;
        type PublicElement = MultiplierCircuitCircuitDefine;
        type PublicWitness = MultiplierCircuitPublicWitness;
        fn create_public(initer: &mut VariableIniter) -> Self::PublicElement {
            MultiplierCircuitCircuitDefine::new(initer)
        }
        fn create_private(initer: &mut VariableIniter) -> Self::PrivateElement {
            MultiplierCircuitCircuitDefine::new(initer)
        }
        fn append_witness(
            &self,
            public: &mut Vec<U256>,
            private: &mut Vec<U256>,
            _is_private: bool,
        ) {
            self.x.append_witness(public, private, true);
            self.y.append_witness(public, private, true);
            self.product.append_witness(public, private, false);
        }
        fn into_public_witness(self) -> Self::PublicWitness {
            MultiplierCircuitPublicWitness {
                product: self.product,
            }
        }
    }
    #[doc(hidden)]
    pub struct MultiplierCircuitCircuitDefine {
        pub x: ::rsnark_core::PrivateCircuitElement<u32>,
        pub y: ::rsnark_core::PrivateCircuitElement<u32>,
        pub product: ::rsnark_core::PublicCircuitElement<u32>,
    }
    impl MultiplierCircuitCircuitDefine {
        fn new(initer: &mut VariableIniter) -> Self {
            let x = <u32 as ::rsnark_core::CircuitWitness>::create_private(initer);
            let y = <u32 as ::rsnark_core::CircuitWitness>::create_private(initer);
            let product = <u32 as ::rsnark_core::CircuitWitness>::create_public(initer);
            Self { x, y, product }
        }
    }
    impl CircuitPublicWitness for MultiplierCircuit {
        fn append_public_witness(&self, witness: &mut Vec<U256>, _is_private: bool) {
            self.x.append_public_witness(witness, true);
            self.y.append_public_witness(witness, true);
            self.product.append_public_witness(witness, false);
        }
    }
    #[doc(hidden)]
    pub struct MultiplierCircuitPublicWitness {
        pub product: u32,
    }
    impl CircuitPublicWitness for MultiplierCircuitPublicWitness {
        fn append_public_witness(&self, witness: &mut Vec<U256>, _is_private: bool) {
            self.product.append_public_witness(witness, false);
        }
    }
}
impl Circuit for CircuitDefine<MultiplierCircuit> {
    fn define(&self, api: &mut impl API) {
        let result = api.mul(&self.x, &self.y);
        api.assert_is_equal(&result, &self.product);
    }
}
pub struct CompositeCircuit {
    adder: AdderCircuit,
    multiplier: MultiplierCircuit,
    pub final_result: u32,
}
mod __rsnark_generated_compositecircuit {
    use super::*;
    use ::rsnark_core::{CircuitPublicWitness, CircuitWitness, U256, VariableIniter};
    impl CircuitWitness for CompositeCircuit {
        type PrivateElement = CompositeCircuitCircuitDefine;
        type PublicElement = CompositeCircuitCircuitDefine;
        type PublicWitness = CompositeCircuitPublicWitness;
        fn create_public(initer: &mut VariableIniter) -> Self::PublicElement {
            CompositeCircuitCircuitDefine::new(initer)
        }
        fn create_private(initer: &mut VariableIniter) -> Self::PrivateElement {
            CompositeCircuitCircuitDefine::new(initer)
        }
        fn append_witness(
            &self,
            public: &mut Vec<U256>,
            private: &mut Vec<U256>,
            _is_private: bool,
        ) {
            self.adder.append_witness(public, private, true);
            self.multiplier.append_witness(public, private, true);
            self.final_result.append_witness(public, private, false);
        }
        fn into_public_witness(self) -> Self::PublicWitness {
            CompositeCircuitPublicWitness {
                final_result: self.final_result,
            }
        }
    }
    #[doc(hidden)]
    pub struct CompositeCircuitCircuitDefine {
        pub adder: ::rsnark_core::PrivateCircuitElement<AdderCircuit>,
        pub multiplier: ::rsnark_core::PrivateCircuitElement<MultiplierCircuit>,
        pub final_result: ::rsnark_core::PublicCircuitElement<u32>,
    }
    impl CompositeCircuitCircuitDefine {
        fn new(initer: &mut VariableIniter) -> Self {
            let adder = <AdderCircuit as ::rsnark_core::CircuitWitness>::create_private(initer);
            let multiplier =
                <MultiplierCircuit as ::rsnark_core::CircuitWitness>::create_private(initer);
            let final_result = <u32 as ::rsnark_core::CircuitWitness>::create_public(initer);
            Self {
                adder,
                multiplier,
                final_result,
            }
        }
    }
    impl CircuitPublicWitness for CompositeCircuit {
        fn append_public_witness(&self, witness: &mut Vec<U256>, _is_private: bool) {
            self.adder.append_public_witness(witness, true);
            self.multiplier.append_public_witness(witness, true);
            self.final_result.append_public_witness(witness, false);
        }
    }
    #[doc(hidden)]
    pub struct CompositeCircuitPublicWitness {
        pub final_result: u32,
    }
    impl CircuitPublicWitness for CompositeCircuitPublicWitness {
        fn append_public_witness(&self, witness: &mut Vec<U256>, _is_private: bool) {
            self.final_result.append_public_witness(witness, false);
        }
    }
}
impl Circuit for CircuitDefine<CompositeCircuit> {
    fn define(&self, api: &mut impl API) {
        self.adder.define(api);
        self.multiplier.define(api);
        let final_sum = api.add(&self.adder.sum, &self.multiplier.product);
        api.assert_is_equal(&final_sum, &self.final_result);
    }
}
fn main() {
    let prover = Groth16BN254GnarkProver::new();
    let circuit_prover = prover.compile_circuit::<CompositeCircuit>().unwrap();
    let (pk, vk) = circuit_prover.setup().unwrap();
    let circuit_witness = CompositeCircuit {
        adder: AdderCircuit { a: 1, b: 2, sum: 3 },
        multiplier: MultiplierCircuit {
            x: 2,
            y: 3,
            product: 6,
        },
        final_result: 9,
    };
    let proof = circuit_prover.prove(&pk, &circuit_witness).unwrap();
    let public_witness = circuit_witness.into_public_witness();
    circuit_prover.verify(&vk, &proof, public_witness).unwrap();
}
