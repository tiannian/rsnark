use ruint::aliases::U256;

use crate::{API, PrivateVariable, PublicVariable, VariableIniter};

pub trait Circuit {
    fn define(&self, api: &mut impl API);
}

pub trait CircuitElement {
    type PrivateElement;
    type PublicElement;

    fn create_public(initer: &mut VariableIniter) -> Self::PublicElement;
    fn create_private(initer: &mut VariableIniter) -> Self::PrivateElement;
    fn append_public(&self, witness: &mut Vec<U256>);
    fn append_private(&self, witness: &mut Vec<U256>);
}

#[doc(hidden)]
pub type PrivateCircuitElement<T> = <T as CircuitElement>::PrivateElement;
#[doc(hidden)]
pub type PublicCircuitElement<T> = <T as CircuitElement>::PublicElement;

pub type CircuitDefine<T> = <T as CircuitElement>::PrivateElement;

macro_rules! define_circuit_element_for_from_u256 {
    ($t:ty) => {
        impl CircuitElement for $t {
            type PrivateElement = PrivateVariable;
            type PublicElement = PublicVariable;

            fn create_public(initer: &mut VariableIniter) -> Self::PublicElement {
                initer.new_public()
            }

            fn create_private(initer: &mut VariableIniter) -> Self::PrivateElement {
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
define_circuit_element_for_from_u256!(i128);
define_circuit_element_for_from_u256!(i64);
define_circuit_element_for_from_u256!(i32);
define_circuit_element_for_from_u256!(i16);
define_circuit_element_for_from_u256!(i8);
define_circuit_element_for_from_u256!(bool);
