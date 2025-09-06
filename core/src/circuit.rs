use ruint::aliases::U256;

use crate::{API, PrivateVariable, PublicVariable, VariableIniter};

pub trait Circuit {
    fn define(&self, api: &mut impl API);
}

pub trait CircuitElement {
    type Private;
    type Public;

    fn create_public(initer: &mut VariableIniter) -> Self::Public;
    fn create_private(initer: &mut VariableIniter) -> Self::Private;
    fn append_public(&self, witness: &mut Vec<U256>);
    fn append_private(&self, witness: &mut Vec<U256>);
}

pub type PrivateCircuitElement<T> = <T as CircuitElement>::Private;
pub type PublicCircuitElement<T> = <T as CircuitElement>::Public;
pub type CircuitDefine<T> = <T as CircuitElement>::Private;

macro_rules! define_circuit_element_for_from_u256 {
    ($t:ty) => {
        impl CircuitElement for $t {
            type Private = PrivateVariable;
            type Public = PublicVariable;

            fn create_public(initer: &mut VariableIniter) -> Self::Public {
                initer.new_public()
            }

            fn create_private(initer: &mut VariableIniter) -> Self::Private {
                initer.new_private()
            }

            fn append_public(&self, witness: &mut Vec<U256>) {
                let x = U256::from(*self);
                witness.push(x);
            }

            fn append_private(&self, witness: &mut Vec<U256>) {
                let x = U256::from(*self);
                witness.push(x);
            }
        }
    };
}

define_circuit_element_for_from_u256!(U256);
define_circuit_element_for_from_u256!(u128);
define_circuit_element_for_from_u256!(u64);
define_circuit_element_for_from_u256!(u32);
define_circuit_element_for_from_u256!(u16);
define_circuit_element_for_from_u256!(u8);
define_circuit_element_for_from_u256!(bool);
