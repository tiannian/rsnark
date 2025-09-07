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
        CircuitElement, PrivateCircuitElement, PublicCircuitElement, U256, VariableIniter,
    };

    impl CircuitElement for DemoCircuit {
        type PrivateElement = DemoCircuitDefine;
        type PublicElement = DemoCircuitDefine;

        fn create_public(initer: &mut VariableIniter) -> Self::PublicElement {
            DemoCircuitDefine::new(initer)
        }

        fn create_private(initer: &mut VariableIniter) -> Self::PrivateElement {
            DemoCircuitDefine::new(initer)
        }

        fn append_public(&self, witness: &mut Vec<U256>) {
            self.c.append_public(witness);
        }

        fn append_private(&self, witness: &mut Vec<U256>) {
            self.a.append_private(witness);
            self.b.append_private(witness);
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
}

fn main() {}
