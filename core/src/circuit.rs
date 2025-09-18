use num::BigInt;

use crate::{API, VariableIniter, variable::CircuitVariable};

/// Defines the logic of an arithmetic circuit for zero-knowledge proofs.
///
/// This trait should be implemented by circuit structures to define their
/// constraint system. The circuit logic is expressed using the provided API
/// operations to build the constraint graph.
pub trait Circuit {
    fn define(&self, api: &mut impl API);
}

/// Defines a circuit with witness data structure.
///
/// This trait should not be implemented manually. Instead, use the
/// `#[derive(Circuit)]` macro to automatically generate the implementation.
/// It provides methods for creating circuit variables and handling witness data.
pub trait CircuitWitness: CircuitPublicWitness {
    type CircuitElement: CircuitElement<CircuitWitness = Self>;
    /// The type representing the public witness for this circuit.
    type PublicWitness: CircuitPublicWitness;

    #[doc(hidden)]
    fn create_public(initer: &mut VariableIniter, is_private: bool) -> Self::CircuitElement;

    #[doc(hidden)]
    fn create_private(initer: &mut VariableIniter) -> Self::CircuitElement;

    /// Converts this circuit witness into its public witness representation.
    ///
    /// This extracts only the public portion of the witness, which is
    /// needed for verification in zero-knowledge proof systems.
    ///
    /// # Returns
    /// The public witness containing only public inputs
    fn into_public_witness(self) -> Self::PublicWitness;

    fn append_witness(&self, public: &mut Vec<BigInt>, private: &mut Vec<BigInt>, is_private: bool);
}

pub trait CircuitElement {
    type CircuitWitness: CircuitWitness;
}

/// Represents the public witness portion of a circuit.
///
/// This trait should not be implemented manually. Instead, use the
/// `#[derive(Circuit)]` macro to automatically generate the implementation.
/// It handles serialization of public inputs for the circuit.
pub trait CircuitPublicWitness {
    fn append_public_witness(&self, witness: &mut Vec<BigInt>, is_private: bool);
}

pub type Witness<T> = <T as CircuitElement>::CircuitWitness;

pub type PublicWitness<T> =
    <<T as CircuitElement>::CircuitWitness as CircuitWitness>::PublicWitness;

pub type CircuitElementInner<T> =
    <<T as CircuitElement>::CircuitWitness as CircuitWitness>::CircuitElement;

macro_rules! define_circuit_element_for_from_u256 {
    ($t:ty) => {
        impl CircuitWitness for $t {
            type CircuitElement = CircuitVariable<$t>;
            type PublicWitness = $t;

            fn create_public(
                initer: &mut VariableIniter,
                is_private: bool,
            ) -> Self::CircuitElement {
                initer.new_public(is_private).into()
            }

            fn create_private(initer: &mut VariableIniter) -> Self::CircuitElement {
                initer.new_private().into()
            }

            fn into_public_witness(self) -> Self::PublicWitness {
                self
            }

            fn append_witness(
                &self,
                public: &mut Vec<BigInt>,
                private: &mut Vec<BigInt>,
                is_private: bool,
            ) {
                let x = BigInt::from(*self);
                if is_private {
                    private.push(x);
                } else {
                    public.push(x);
                }
            }
        }

        impl CircuitPublicWitness for $t {
            fn append_public_witness(&self, witness: &mut Vec<BigInt>, is_private: bool) {
                let x = BigInt::from(*self);
                if !is_private {
                    witness.push(x);
                }
            }
        }

        impl CircuitElement for CircuitVariable<$t> {
            type CircuitWitness = $t;
        }

        impl CircuitElement for $t {
            type CircuitWitness = $t;
        }
    };
}

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
