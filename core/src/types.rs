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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "t", content = "v")]
pub enum VariableType {
    Public(u64),
    Private(u64),
    Constant(BigInt),
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
