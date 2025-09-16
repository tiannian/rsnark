use rsnark::{
    Groth16BN254GnarkProver,
    core::{API, Circuit, CircuitDefine, CircuitWitness, curve::BN254},
};
use rsnark_core::Variable;
use rsnark_provers_core::Backend;
use rsnark_provers_gnark::Groth16Backend;
pub struct TestCircuit<T> {
    a: T,
    b: T,
    pub c: T,
}
mod __rsnark_generated_testcircuit {
    use super::*;
    use ::rsnark_core::{BigInt, CircuitPublicWitness, CircuitWitness, VariableIniter};
    use rsnark_core::Variable;
    impl<T> CircuitWitness for TestCircuit<T>
    where
        T: ::rsnark_core::CircuitWitness,
    {
        type PrivateElement = TestCircuitCircuitDefine<T>;
        type PublicElement = TestCircuitCircuitDefine<T>;
        type PublicWitness = TestCircuitPublicWitness<T>;
        fn create_public(initer: &mut VariableIniter, is_private: bool) -> Self::PublicElement {
            TestCircuitCircuitDefine::new(initer, is_private)
        }
        fn create_private(initer: &mut VariableIniter) -> Self::PrivateElement {
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
    pub struct TestCircuitCircuitDefine<T>
    where
        T: ::rsnark_core::CircuitWitness,
    {
        pub a: ::rsnark_core::PrivateCircuitElement<T>,
        pub b: ::rsnark_core::PrivateCircuitElement<T>,
        pub c: ::rsnark_core::PublicCircuitElement<T>,
    }
    impl<T> TestCircuitCircuitDefine<T>
    where
        T: ::rsnark_core::CircuitWitness,
    {
        fn new(initer: &mut VariableIniter, is_private: bool) -> Self {
            let a = T::create_private(initer);
            let b = T::create_private(initer);
            let c = T::create_public(initer, is_private);
            Self { a, b, c }
        }
    }
    impl<T> CircuitPublicWitness for TestCircuit<T>
    where
        T: ::rsnark_core::CircuitWitness,
    {
        fn append_public_witness(&self, witness: &mut Vec<BigInt>, _is_private: bool) {
            self.c.append_public_witness(witness, false);
        }
    }
    #[doc(hidden)]
    pub struct TestCircuitPublicWitness<T>
    where
        T: ::rsnark_core::CircuitWitness,
    {
        pub c: ::rsnark_core::PublicWitness<T>,
    }
    impl<T> CircuitPublicWitness for TestCircuitPublicWitness<T>
    where
        T: ::rsnark_core::CircuitWitness,
    {
        fn append_public_witness(&self, witness: &mut Vec<BigInt>, _is_private: bool) {
            self.c.append_public_witness(witness, false);
        }
    }
}

impl<T> Circuit for CircuitDefine<TestCircuit<T>>
where
    T: CircuitWitness,
    T::PublicElement: Variable,
    T::PrivateElement: Variable,
{
    fn define(&self, api: &mut impl API) {
        let c = api.add(&self.a, &self.b);
        api.assert_is_equal(&c, &self.c);
    }
}

fn main() {}
