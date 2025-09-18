//! Variable types for circuit construction.
//!
//! These types can used in API to define circuit.

use std::marker::PhantomData;

use num::BigInt;

use crate::types::VariableType;

/// A trait that defines types that can be used as API parameters in circuit construction.
///
/// This trait enables various types to be used interchangeably in circuit operations,
/// including public variables, private variables, local variables, and constant values.
/// Each implementor must provide its variable type representation.
pub trait Variable {
    /// Returns the variable type representation for this variable.
    /// This is used internally by the circuit builder to track variable usage.
    fn ty(&self) -> VariableType;
}

impl Variable for VariableType {
    fn ty(&self) -> VariableType {
        self.clone()
    }
}

pub struct CircuitVariable<T> {
    variable: VariableType,
    marker: PhantomData<T>,
}

impl<T> Variable for CircuitVariable<T> {
    fn ty(&self) -> VariableType {
        self.variable.clone()
    }
}

impl<T> From<VariableType> for CircuitVariable<T> {
    fn from(variable: VariableType) -> Self {
        Self {
            variable,
            marker: PhantomData,
        }
    }
}

macro_rules! define_variable_for_from_u256 {
    ($t:ident) => {
        impl Variable for $t {
            fn ty(&self) -> VariableType {
                let x = BigInt::from(*self);
                VariableType::Constant(x)
            }
        }
    };
}

define_variable_for_from_u256!(u128);
define_variable_for_from_u256!(u64);
define_variable_for_from_u256!(u32);
define_variable_for_from_u256!(u16);
define_variable_for_from_u256!(u8);
define_variable_for_from_u256!(i128);
define_variable_for_from_u256!(i64);
define_variable_for_from_u256!(i32);
define_variable_for_from_u256!(i16);
define_variable_for_from_u256!(i8);
define_variable_for_from_u256!(bool);
