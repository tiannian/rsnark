//! Some useful types for build prover.
//!
//! Note: These types are used for build prover, so they are not part of the public API.

use num::BigInt;
use serde::{Deserialize, Serialize};
use serde_with::{DisplayFromStr, serde_as};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CircuitDefinition {
    pub private_len: u64,
    pub public_len: u64,
    pub local_len: u64,
    pub operations: Vec<Operation>,
}

/// Enumeration of all supported circuit operations.
///
/// This enum defines the complete set of operations that can be performed
/// within a zero-knowledge circuit. Each operation corresponds to a specific
/// constraint or computation that the prover must satisfy.
///
/// # Operation Categories
///
/// ## Arithmetic Operations
/// - [`Add`](OpCode::Add): Addition of two field elements
/// - [`Sub`](OpCode::Sub): Subtraction of two field elements  
/// - [`Mul`](OpCode::Mul): Multiplication of two field elements
/// - [`MulAcc`](OpCode::MulAcc): Multiply-accumulate operation
/// - [`Neg`](OpCode::Neg): Negation of a field element
/// - [`Div`](OpCode::Div): Field division with constraints
/// - [`DivUnchecked`](OpCode::DivUnchecked): Field division without zero checks
/// - [`Inverse`](OpCode::Inverse): Multiplicative inverse
///
/// ## Bitwise Operations
/// - [`Xor`](OpCode::Xor): Bitwise XOR
/// - [`Or`](OpCode::Or): Bitwise OR
/// - [`And`](OpCode::And): Bitwise AND
/// - [`ToBinary`](OpCode::ToBinary): Convert field element to binary representation
/// - [`FromBinary`](OpCode::FromBinary): Convert binary representation to field element
///
/// ## Comparison and Logic
/// - [`Cmp`](OpCode::Cmp): Compare two values
/// - [`IsZero`](OpCode::IsZero): Check if value is zero
/// - [`Select`](OpCode::Select): Conditional selection (ternary operator)
///
/// ## Lookup Operations
/// - [`Lookup2`](OpCode::Lookup2): 2-input lookup table operation
///
/// ## Assertion Operations
/// - [`AssertIsEqual`](OpCode::AssertIsEqual): Assert two values are equal
/// - [`AssertIsDifferent`](OpCode::AssertIsDifferent): Assert two values are different
/// - [`AssertIsBoolean`](OpCode::AssertIsBoolean): Assert value is boolean (0 or 1)
/// - [`AssertIsCrumb`](OpCode::AssertIsCrumb): Assert value is a 2-bit value (0, 1, 2, or 3)
/// - [`AssertIsLessOrEqual`](OpCode::AssertIsLessOrEqual): Assert first value ≤ second value
///
/// ## Debug Operations
/// - [`Println`](OpCode::Println): Print value for debugging (backend-dependent)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OpCode {
    Add,
    MulAcc,
    Neg,
    Sub,
    Mul,
    DivUnchecked,
    Div,
    Inverse,
    ToBinary,
    FromBinary,
    Xor,
    Or,
    And,
    Select,
    Lookup2,
    IsZero,
    Cmp,
    AssertIsEqual,
    AssertIsDifferent,
    AssertIsBoolean,
    AssertIsCrumb,
    AssertIsLessOrEqual,
    Println,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Operation {
    pub op: OpCode,
    #[serde(rename = "in")]
    pub inputs: Vec<VariableType>,
    #[serde(rename = "out")]
    pub outputs: Vec<VariableType>,
}

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "t", content = "v")]
pub enum VariableType {
    Public(u64),
    Private(u64),
    Constant(#[serde_as(as = "DisplayFromStr")] BigInt),
    Local(u64),
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
pub struct Witness {
    #[serde_as(as = "Vec<DisplayFromStr>")]
    public: Vec<BigInt>,
    #[serde_as(as = "Vec<DisplayFromStr>")]
    private: Vec<BigInt>,
}

impl Default for Witness {
    fn default() -> Self {
        Self::new()
    }
}

impl From<(Vec<BigInt>, Vec<BigInt>)> for Witness {
    fn from((public, private): (Vec<BigInt>, Vec<BigInt>)) -> Self {
        Self { public, private }
    }
}

impl Witness {
    pub fn new() -> Self {
        Self {
            public: Vec::new(),
            private: Vec::new(),
        }
    }

    pub fn to_public(&self) -> PublicWitness {
        PublicWitness {
            public: self.public.clone(),
        }
    }

    pub fn public_mut(&mut self) -> &mut Vec<BigInt> {
        &mut self.public
    }

    pub fn private_mut(&mut self) -> &mut Vec<BigInt> {
        &mut self.private
    }

    pub fn public(&self) -> &[BigInt] {
        &self.public
    }

    pub fn private(&self) -> &[BigInt] {
        &self.private
    }
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
pub struct PublicWitness {
    #[serde_as(as = "Vec<DisplayFromStr>")]
    pub public: Vec<BigInt>,
}

impl Default for PublicWitness {
    fn default() -> Self {
        Self::new()
    }
}

impl PublicWitness {
    pub fn new() -> Self {
        Self { public: Vec::new() }
    }

    pub fn public_mut(&mut self) -> &mut Vec<BigInt> {
        &mut self.public
    }
}
