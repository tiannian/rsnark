use ruint::aliases::U256;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CircuitDefinition {
    pub private_len: u64,
    pub public_len: u64,
    pub local_len: u64,
    pub operations: Vec<Operation>,
}

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
pub struct Operation {
    pub op: OpCode,
    #[serde(rename = "in")]
    pub inputs: Vec<VariableType>,
    #[serde(rename = "out")]
    pub outputs: Vec<VariableType>,
}

#[derive(Debug, Serialize, Deserialize)]
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
    pub public: Vec<U256>,
    pub private: Vec<U256>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PublicWitness {
    pub public: Vec<U256>,
}
