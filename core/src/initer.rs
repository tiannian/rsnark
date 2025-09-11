use crate::{types::VariableType, variable::CircuitVariable};

#[doc(hidden)]
#[derive(Debug, Default)]
pub struct VariableIniter {
    private_idx: u64,
    public_idx: u64,
    local_idx: u64,
}

impl VariableIniter {
    pub fn new_private(&mut self) -> CircuitVariable {
        let idx = self.private_idx;
        self.private_idx += 1;
        CircuitVariable {
            ty: VariableType::Private(idx),
        }
    }

    pub fn new_public(&mut self, is_private: bool) -> CircuitVariable {
        if is_private {
            self.new_private()
        } else {
            let idx = self.public_idx;
            self.public_idx += 1;

            CircuitVariable {
                ty: VariableType::Public(idx),
            }
        }
    }

    pub fn new_local(&mut self) -> CircuitVariable {
        let idx = self.local_idx;
        self.local_idx += 1;
        CircuitVariable {
            ty: VariableType::Local(idx),
        }
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
