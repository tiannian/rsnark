use crate::{
    API, Metadata, MetadataInfo, Variable, VariableIniter,
    types::{CircuitDefinition, OpCode, Operation, VariableType},
    variable::CircuitVariable,
};

#[doc(hidden)]
#[derive(Debug)]
pub struct CircuitBuilder {
    operations: Vec<Operation>,
    variable_initer: VariableIniter,
    metadata: MetadataInfo,
}

impl CircuitBuilder {
    pub fn new(metadata: MetadataInfo) -> Self {
        Self {
            operations: Vec::new(),
            variable_initer: VariableIniter::default(),
            metadata,
        }
    }

    pub fn variable_initer(&self) -> &VariableIniter {
        &self.variable_initer
    }

    pub fn variable_initer_mut(&mut self) -> &mut VariableIniter {
        &mut self.variable_initer
    }

    fn _append_operation(
        &mut self,
        op: OpCode,
        inputs: Vec<VariableType>,
        outputs: Vec<CircuitVariable>,
    ) {
        let mut _outs = Vec::with_capacity(outputs.len());
        for output in outputs {
            _outs.push(output.ty());
        }

        let operation = Operation {
            op,
            inputs,
            outputs: _outs,
        };

        self.operations.push(operation);
    }

    fn _allocate_local_variable(&mut self) -> CircuitVariable {
        self.variable_initer.new_local()
    }

    pub fn build(self) -> CircuitDefinition {
        CircuitDefinition {
            private_len: self.variable_initer.private_index(),
            public_len: self.variable_initer.public_index(),
            local_len: self.variable_initer.local_index(),
            operations: self.operations,
        }
    }
}

impl API for CircuitBuilder {
    fn metadata(&self) -> &impl Metadata {
        &self.metadata
    }

    fn append_operation(
        &mut self,
        op: OpCode,
        inputs: Vec<VariableType>,
        outputs: Vec<CircuitVariable>,
    ) {
        self._append_operation(op, inputs, outputs);
    }

    fn allocate_local_variable(&mut self) -> CircuitVariable {
        self._allocate_local_variable()
    }
}
