use rsnark::{
    Groth16BN254GnarkProver,
    core::{API, Circuit, CircuitWitness, curve::BN254},
};
use rsnark_core::{CircuitElement, Variable, types::VariableType};
use rsnark_provers_core::Backend;
use rsnark_provers_gnark::Groth16Backend;
pub struct TestCircuit<T>
where
    T: CircuitWitness,
{
    a: T::CircuitElement,
    b: T::CircuitElement,
    pub c: T::CircuitElement,
}
mod __rsnark_generated_testcircuit {
    use super::*;
    use ::rsnark_core::{
        BigInt, CircuitPublicWitness, CircuitWitness, VariableIniter, types::VariableType,
    };

    impl<T> TestCircuit<T>
    where
        T: CircuitWitness<CircuitElement = VariableType>,
    {
        fn new(initer: &mut VariableIniter, is_private: bool) -> Self {
            let a = u32::create_private(initer);
            let b = u32::create_private(initer);
            let c = u32::create_public(initer, is_private);
            Self { a, b, c }
        }
    }

    impl<T> CircuitElement for TestCircuit<T>
    where
        T: CircuitWitness<CircuitElement = VariableType>,
    {
        type CircuitWitness = TestCircuitWitness<T>;
    }

    pub struct TestCircuitWitness<T> {
        pub a: T,
        pub b: T,
        pub c: T,
    }

    impl<T> CircuitPublicWitness for TestCircuitWitness<T>
    where
        T: CircuitWitness<CircuitElement = VariableType>,
    {
        fn append_public_witness(&self, witness: &mut Vec<BigInt>, _is_private: bool) {
            self.c.append_public_witness(witness, false);
        }
    }

    impl<T> CircuitWitness for TestCircuitWitness<T>
    where
        T: CircuitWitness<CircuitElement = VariableType>,
    {
        type CircuitElement = TestCircuit<T>;
        type PublicWitness = TestCircuitPublicWitness<T>;
        fn create_public(initer: &mut VariableIniter, is_private: bool) -> Self::CircuitElement {
            TestCircuit::new(initer, is_private)
        }
        fn create_private(initer: &mut VariableIniter) -> Self::CircuitElement {
            TestCircuit::new(initer, true)
        }
        fn append_witness(
            &self,
            public: &mut Vec<BigInt>,
            private: &mut Vec<BigInt>,
            _is_private: bool,
        ) {
            self.a.append_witness(public, private, true);
            self.b.append_witness(public, private, true);
            self.c.append_witness(public, private, false);
        }
        fn into_public_witness(self) -> Self::PublicWitness {
            TestCircuitPublicWitness {
                c: self.c.into_public_witness(),
            }
        }
    }

    #[doc(hidden)]
    pub struct TestCircuitPublicWitness<T: CircuitWitness> {
        pub c: T::PublicWitness,
    }
    impl<T> CircuitPublicWitness for TestCircuitPublicWitness<T>
    where
        T: CircuitWitness<CircuitElement = VariableType>,
    {
        fn append_public_witness(&self, witness: &mut Vec<BigInt>, _is_private: bool) {
            self.c.append_public_witness(witness, false);
        }
    }
}

impl<T> Circuit for TestCircuit<T>
where
    T: CircuitWitness<CircuitElement = VariableType>,
{
    fn define(&self, api: &mut impl API) {
        let c = api.add(&self.a, &self.b);
        api.assert_is_equal(&c, &self.c);
    }
}
fn main() {}
