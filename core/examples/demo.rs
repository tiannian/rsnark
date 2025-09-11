use rsnark_core::{API, Circuit, CircuitDefine};

// #[derive(Circuit)]
pub struct DemoCircuit {
    a: u32,
    b: u32,
    pub c: u32,
}

impl Circuit for CircuitDefine<DemoCircuit> {
    fn define(&self, api: &mut impl API) {
        let c = api.add(&self.a, &self.b);

        api.assert_is_equal(&c, &self.c);
    }
}

mod __rsnark_generated_demo {
    use super::*;

    use ::rsnark_core::{
        CircuitPublicWitness, CircuitWitness, PrivateCircuitElement, PublicCircuitElement, U256,
        VariableIniter,
    };

    impl CircuitWitness for DemoCircuit {
        type PrivateElement = DemoCircuitDefine;
        type PublicElement = DemoCircuitDefine;
        type PublicWitness = DemoCircuitPublicWitness;

        fn create_public(initer: &mut VariableIniter) -> Self::PublicElement {
            DemoCircuitDefine::new(initer)
        }

        fn create_private(initer: &mut VariableIniter) -> Self::PrivateElement {
            DemoCircuitDefine::new(initer)
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
            DemoCircuitPublicWitness { c: self.c }
        }
    }

    impl CircuitPublicWitness for DemoCircuit {
        fn append_public_witness(&self, witness: &mut Vec<U256>, _is_private: bool) {
            self.c.append_public_witness(witness, false);
        }
    }

    #[doc(hidden)]
    pub struct DemoCircuitDefine {
        pub a: PrivateCircuitElement<u32>,
        pub b: PrivateCircuitElement<u32>,
        pub c: PublicCircuitElement<u32>,
    }

    impl DemoCircuitDefine {
        fn new(initer: &mut VariableIniter) -> Self {
            let a = u32::create_private(initer);
            let b = u32::create_private(initer);
            let c = u32::create_public(initer);

            Self { a, b, c }
        }
    }

    #[doc(hidden)]
    pub struct DemoCircuitPublicWitness {
        pub c: u32,
    }

    impl CircuitPublicWitness for DemoCircuitPublicWitness {
        fn append_public_witness(&self, witness: &mut Vec<U256>, _is_private: bool) {
            self.c.append_public_witness(witness, false);
        }
    }
}

fn main() {}
