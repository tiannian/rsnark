use ruint::aliases::U256;

use crate::types::VariableType;

pub trait Variable {
    fn ty(&self) -> VariableType;
}

#[derive(Debug, Clone)]
pub struct PublicVariable {
    pub(crate) index: u64,
}

impl Variable for PublicVariable {
    fn ty(&self) -> VariableType {
        VariableType::Public(self.index)
    }
}

#[derive(Debug, Clone)]
pub struct PrivateVariable {
    pub(crate) index: u64,
}

impl Variable for PrivateVariable {
    fn ty(&self) -> VariableType {
        VariableType::Private(self.index)
    }
}

#[derive(Debug, Clone)]
pub struct LocalVariable {
    pub(crate) index: u64,
}

impl Variable for LocalVariable {
    fn ty(&self) -> VariableType {
        VariableType::Local(self.index)
    }
}

impl Variable for U256 {
    fn ty(&self) -> VariableType {
        VariableType::Constant(*self)
    }
}

impl Variable for u128 {
    fn ty(&self) -> VariableType {
        let x = U256::from(*self);
        VariableType::Constant(x)
    }
}

impl Variable for u64 {
    fn ty(&self) -> VariableType {
        let x = U256::from(*self);
        VariableType::Constant(x)
    }
}

impl Variable for bool {
    fn ty(&self) -> VariableType {
        let x = U256::from(*self);
        VariableType::Constant(x)
    }
}

#[derive(Debug, Default)]
pub struct VariableIniter {
    private_idx: u64,
    public_idx: u64,
    local_idx: u64,
}

impl VariableIniter {
    pub fn new_private(&mut self) -> PrivateVariable {
        let idx = self.private_idx;
        self.private_idx += 1;
        PrivateVariable { index: idx }
    }

    pub fn new_public(&mut self) -> PublicVariable {
        let idx = self.public_idx;
        self.public_idx += 1;
        PublicVariable { index: idx }
    }

    pub fn new_local(&mut self) -> LocalVariable {
        let idx = self.local_idx;
        self.local_idx += 1;
        LocalVariable { index: idx }
    }

    pub fn private_index(&self) -> u64 {
        self.private_idx
    }

    pub fn public_index(&self) -> u64 {
        self.public_idx
    }

    pub fn local_index(&self) -> u64 {
        self.local_idx
    }
}
