//! Variable types and constant value support for circuit construction.
//!
//! This module provides the foundation for rSnark's flexible variable system,
//! including support for BigInt constants and automatic type conversion.
//!
//! # Key Features
//!
//! - **Automatic constant conversion**: Primitive types are automatically converted to BigInt constants
//! - **Type safety**: Strong typing ensures correct variable usage in circuits
//! - **Generic support**: Works with generic circuit structures
//! - **BigInt precision**: Support for arbitrary precision arithmetic
//!
//! # Supported Constant Types
//!
//! The following primitive types are automatically converted to circuit constants:
//! - Unsigned integers: `u8`, `u16`, `u32`, `u64`, `u128`
//! - Signed integers: `i8`, `i16`, `i32`, `i64`, `i128`  
//! - Boolean: `bool`
//!
//! # Usage in Circuits
//!
//! ```rust,ignore
//! impl Circuit for MyCircuit {
//!     fn define(&self, api: &mut impl API) {
//!         // All these literals are automatically converted to BigInt constants
//!         let result = api.add(&self.input, &42_u32);
//!         let scaled = api.mul(&result, &1000_i64);
//!         let flag = api.select(&true, &result, &0);
//!     }
//! }
//! ```

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

/// Macro to automatically implement Variable trait for primitive types.
///
/// This macro generates implementations that convert primitive values to BigInt constants.
/// The conversion preserves the original value's precision and sign, enabling seamless
/// integration of literal values in circuit operations.
///
/// # Generated Implementation
///
/// For each type `T`, this generates:
/// ```rust,ignore
/// impl Variable for T {
///     fn ty(&self) -> VariableType {
///         let x = BigInt::from(*self);
///         VariableType::Constant(x)
///     }
/// }
/// ```
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
