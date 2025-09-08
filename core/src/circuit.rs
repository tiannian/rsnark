use ruint::aliases::U256;

use crate::{API, PrivateVariable, PublicVariable, VariableIniter};

pub trait Circuit {
    fn define(&self, api: &mut impl API);
}

pub trait CircuitWitness: CircuitPublicWitness {
    type PrivateElement;
    type PublicElement;
    type PublicWitness: CircuitPublicWitness;

    fn create_public(initer: &mut VariableIniter) -> Self::PublicElement;

    fn create_private(initer: &mut VariableIniter) -> Self::PrivateElement;

    fn into_public_witness(self) -> Self::PublicWitness;

    fn append_private(&self, witness: &mut Vec<U256>);
}

pub trait CircuitPublicWitness {
    fn append_public(&self, witness: &mut Vec<U256>);
}

#[doc(hidden)]
pub type PrivateCircuitElement<T> = <T as CircuitWitness>::PrivateElement;
#[doc(hidden)]
pub type PublicCircuitElement<T> = <T as CircuitWitness>::PublicElement;

pub type CircuitDefine<T> = <T as CircuitWitness>::PrivateElement;

pub type PublicWitness<T> = <T as CircuitWitness>::PublicWitness;

macro_rules! define_circuit_element_for_from_u256 {
    ($t:ty) => {
        impl CircuitWitness for $t {
            type PrivateElement = PrivateVariable;
            type PublicElement = PublicVariable;
            type PublicWitness = $t;

            fn create_public(initer: &mut VariableIniter) -> Self::PublicElement {
                initer.new_public()
            }

            fn create_private(initer: &mut VariableIniter) -> Self::PrivateElement {
                initer.new_private()
            }

            fn append_private(&self, witness: &mut Vec<U256>) {
                let x = U256::from(*self);
                witness.push(x);
            }

            fn into_public_witness(self) -> Self::PublicWitness {
                self
            }
        }

        impl CircuitPublicWitness for $t {
            fn append_public(&self, witness: &mut Vec<U256>) {
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
