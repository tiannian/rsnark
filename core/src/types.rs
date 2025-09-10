use ruint::aliases::U256;
use serde::{Deserialize, Serialize};

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
    Constant(U256),
    Local(u64),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Witness {
    public: Vec<U256>,
    private: Vec<U256>,
}

impl Default for Witness {
    fn default() -> Self {
        Self::new()
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

    pub fn public_mut(&mut self) -> &mut Vec<U256> {
        &mut self.public
    }

    pub fn private_mut(&mut self) -> &mut Vec<U256> {
        &mut self.private
    }

    pub fn public(&self) -> &[U256] {
        &self.public
    }

    pub fn private(&self) -> &[U256] {
        &self.private
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PublicWitness {
    pub public: Vec<U256>,
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

    pub fn public_mut(&mut self) -> &mut Vec<U256> {
        &mut self.public
    }
}
