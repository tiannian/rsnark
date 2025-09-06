use crate::{LocalVariable, PrivateVariable, PublicVariable};

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
