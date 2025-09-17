use rsnark::PlonkBN254GnarkProver;
use rsnark_core::{API, Circuit, CircuitWitness, circuit};
#[doc(hidden)]
pub struct AdderCircuit {
    a: <u32 as ::rsnark_core::CircuitWitness>::CircuitElement,
    b: <u32 as ::rsnark_core::CircuitWitness>::CircuitElement,
    pub sum: <u32 as ::rsnark_core::CircuitWitness>::CircuitElement,
}
mod __rsnark_generated_addercircuit {
    use super::*;
    use ::rsnark_core::{BigInt, CircuitPublicWitness, CircuitWitness, VariableIniter};
    use rsnark_core::CircuitElement;
    impl AdderCircuit {
        fn new(initer: &mut VariableIniter, is_private: bool) -> Self {
            let a = u32::create_private(initer);
            let b = u32::create_private(initer);
            let sum = u32::create_public(initer, is_private);
            Self { a, b, sum }
        }
    }
    impl CircuitElement for AdderCircuit {
        type CircuitWitness = AdderCircuitWitness;
    }
    pub struct AdderCircuitWitness {
        pub a: u32,
        pub b: u32,
        pub sum: u32,
    }
    impl CircuitWitness for AdderCircuitWitness {
        type CircuitElement = AdderCircuit;
        type PublicWitness = AdderCircuitPublicWitness;
        fn create_public(initer: &mut VariableIniter, is_private: bool) -> Self::CircuitElement {
            AdderCircuit::new(initer, is_private)
        }
        fn create_private(initer: &mut VariableIniter) -> Self::CircuitElement {
            AdderCircuit::new(initer, true)
        }
        fn append_witness(
            &self,
            public: &mut Vec<BigInt>,
            private: &mut Vec<BigInt>,
            _is_private: bool,
        ) {
            self.a.append_witness(public, private, true);
            self.b.append_witness(public, private, true);
            self.sum
                .append_witness(public, private, false || _is_private);
        }
        fn into_public_witness(self) -> Self::PublicWitness {
            AdderCircuitPublicWitness {
                sum: self.sum.into_public_witness(),
            }
        }
    }
    impl CircuitPublicWitness for AdderCircuitWitness {
        fn append_public_witness(&self, witness: &mut Vec<BigInt>, _is_private: bool) {
            self.sum.append_public_witness(witness, false);
        }
    }
    #[doc(hidden)]
    pub struct AdderCircuitPublicWitness {
        pub sum: <u32 as ::rsnark_core::CircuitWitness>::PublicWitness,
    }
    impl CircuitPublicWitness for AdderCircuitPublicWitness {
        fn append_public_witness(&self, witness: &mut Vec<BigInt>, _is_private: bool) {
            self.sum.append_public_witness(witness, false);
        }
    }
}
impl Circuit for AdderCircuit {
    fn define(&self, api: &mut impl API) {
        let result = api.add(&self.a, &self.b);
        api.assert_is_equal(&result, &self.sum);
    }
}
#[doc(hidden)]
pub struct MultiplierCircuit {
    x: <u32 as ::rsnark_core::CircuitWitness>::CircuitElement,
    y: <u32 as ::rsnark_core::CircuitWitness>::CircuitElement,
    pub product: <u32 as ::rsnark_core::CircuitWitness>::CircuitElement,
}
mod __rsnark_generated_multipliercircuit {
    use super::*;
    use ::rsnark_core::{BigInt, CircuitPublicWitness, CircuitWitness, VariableIniter};
    use rsnark_core::CircuitElement;
    impl MultiplierCircuit {
        fn new(initer: &mut VariableIniter, is_private: bool) -> Self {
            let x = u32::create_private(initer);
            let y = u32::create_private(initer);
            let product = u32::create_public(initer, is_private);
            Self { x, y, product }
        }
    }
    impl CircuitElement for MultiplierCircuit {
        type CircuitWitness = MultiplierCircuitWitness;
    }
    pub struct MultiplierCircuitWitness {
        pub x: u32,
        pub y: u32,
        pub product: u32,
    }
    impl CircuitWitness for MultiplierCircuitWitness {
        type CircuitElement = MultiplierCircuit;
        type PublicWitness = MultiplierCircuitPublicWitness;
        fn create_public(initer: &mut VariableIniter, is_private: bool) -> Self::CircuitElement {
            MultiplierCircuit::new(initer, is_private)
        }
        fn create_private(initer: &mut VariableIniter) -> Self::CircuitElement {
            MultiplierCircuit::new(initer, true)
        }
        fn append_witness(
            &self,
            public: &mut Vec<BigInt>,
            private: &mut Vec<BigInt>,
            _is_private: bool,
        ) {
            self.x.append_witness(public, private, true);
            self.y.append_witness(public, private, true);
            self.product
                .append_witness(public, private, false || _is_private);
        }
        fn into_public_witness(self) -> Self::PublicWitness {
            MultiplierCircuitPublicWitness {
                product: self.product.into_public_witness(),
            }
        }
    }
    impl CircuitPublicWitness for MultiplierCircuitWitness {
        fn append_public_witness(&self, witness: &mut Vec<BigInt>, _is_private: bool) {
            self.product.append_public_witness(witness, false);
        }
    }
    #[doc(hidden)]
    pub struct MultiplierCircuitPublicWitness {
        pub product: <u32 as ::rsnark_core::CircuitWitness>::PublicWitness,
    }
    impl CircuitPublicWitness for MultiplierCircuitPublicWitness {
        fn append_public_witness(&self, witness: &mut Vec<BigInt>, _is_private: bool) {
            self.product.append_public_witness(witness, false);
        }
    }
}
impl Circuit for MultiplierCircuit {
    fn define(&self, api: &mut impl API) {
        let result = api.mul(&self.x, &self.y);
        api.assert_is_equal(&result, &self.product);
    }
}
#[doc(hidden)]
pub struct CompositeCircuit {
    adder: ::rsnark_core::CircuitElementInner<AdderCircuit>,
    multiplier: ::rsnark_core::CircuitElementInner<MultiplierCircuit>,
    pub final_result: <u32 as ::rsnark_core::CircuitWitness>::CircuitElement,
}
mod __rsnark_generated_compositecircuit {
    use super::*;
    use ::rsnark_core::{BigInt, CircuitPublicWitness, CircuitWitness, VariableIniter};
    use rsnark_core::CircuitElement;
    impl CompositeCircuit {
        fn new(initer: &mut VariableIniter, is_private: bool) -> Self {
            let adder = ::rsnark_core::Witness::<AdderCircuit>::create_private(initer);
            let multiplier = ::rsnark_core::Witness::<MultiplierCircuit>::create_private(initer);
            let final_result = ::rsnark_core::Witness::<u32>::create_public(initer, is_private);
            Self {
                adder,
                multiplier,
                final_result,
            }
        }
    }
    impl CircuitElement for CompositeCircuit {
        type CircuitWitness = CompositeCircuitWitness;
    }
    pub struct CompositeCircuitWitness {
        pub adder: AdderCircuit,
        pub multiplier: MultiplierCircuit,
        pub final_result: u32,
    }
    impl CircuitWitness for CompositeCircuitWitness {
        type CircuitElement = CompositeCircuit;
        type PublicWitness = CompositeCircuitPublicWitness;
        fn create_public(initer: &mut VariableIniter, is_private: bool) -> Self::CircuitElement {
            CompositeCircuit::new(initer, is_private)
        }
        fn create_private(initer: &mut VariableIniter) -> Self::CircuitElement {
            CompositeCircuit::new(initer, true)
        }
        fn append_witness(
            &self,
            public: &mut Vec<BigInt>,
            private: &mut Vec<BigInt>,
            _is_private: bool,
        ) {
            self.adder.append_witness(public, private, true);
            self.multiplier.append_witness(public, private, true);
            self.final_result
                .append_witness(public, private, false || _is_private);
        }
        fn into_public_witness(self) -> Self::PublicWitness {
            CompositeCircuitPublicWitness {
                final_result: self.final_result.into_public_witness(),
            }
        }
    }
    impl CircuitPublicWitness for CompositeCircuitWitness {
        fn append_public_witness(&self, witness: &mut Vec<BigInt>, _is_private: bool) {
            self.final_result.append_public_witness(witness, false);
        }
    }
    #[doc(hidden)]
    pub struct CompositeCircuitPublicWitness {
        pub final_result: <u32 as ::rsnark_core::CircuitWitness>::PublicWitness,
    }
    impl CircuitPublicWitness for CompositeCircuitPublicWitness {
        fn append_public_witness(&self, witness: &mut Vec<BigInt>, _is_private: bool) {
            self.final_result.append_public_witness(witness, false);
        }
    }
}
fn main() {}
