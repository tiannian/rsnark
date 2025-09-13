//! Variable types for circuit construction.
//!
//! These types can used in API to define circuit.

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

/// Represents a variable within a circuit during construction.
///
/// A `CircuitVariable` is created during circuit building and encapsulates a specific
/// variable type (public, private, local, or constant). It serves as a handle to reference
/// variables in circuit operations and maintains the variable's type information internally.
///
/// # Usage
///
/// Circuit variables are typically created through:
/// - [`VariableIniter`](crate::VariableIniter) methods for creating new variables
/// - API operations that return intermediate results
/// - Circuit builder operations that allocate local variables
///
/// # Implementation Details
///
/// The struct contains a [`VariableType`] that specifies whether
/// the variable is public input, private witness, local intermediate value, or a constant.
/// This type information is used by the circuit builder to generate proper constraints.
#[derive(Debug, Clone)]
pub struct CircuitVariable {
    pub(crate) ty: VariableType,
}

impl Variable for CircuitVariable {
    fn ty(&self) -> VariableType {
        self.ty.clone()
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
