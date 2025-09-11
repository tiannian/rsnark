use crate::{
    API, Variable, VariableIniter,
    types::{CircuitDefinition, OpCode, Operation, VariableType},
    variable::CircuitVariable,
};

#[doc(hidden)]
#[derive(Debug, Default)]
pub struct CircuitBuilder {
    operations: Vec<Operation>,
    variable_initer: VariableIniter,
}

impl CircuitBuilder {
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

    fn _allocate_local_variable_n(&mut self, n: u64) -> Vec<CircuitVariable> {
        let mut res = Vec::with_capacity(n as usize);
        for _ in 0..n {
            res.push(self._allocate_local_variable());
        }
        res
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
    fn add_multi(
        &mut self,
        x1: &impl Variable,
        x2: &impl Variable,
        xn: &[&dyn Variable],
    ) -> CircuitVariable {
        let res = self._allocate_local_variable();

        self._append_operation(
            OpCode::Add,
            get_variable_type_2n(x1, x2, xn),
            vec![res.clone()],
        );

        res
    }

    fn mul_acc(
        &mut self,
        a: &impl Variable,
        b: &impl Variable,
        c: &impl Variable,
    ) -> CircuitVariable {
        let res = self._allocate_local_variable();

        self._append_operation(
            OpCode::MulAcc,
            vec![a.ty(), b.ty(), c.ty()],
            vec![res.clone()],
        );

        res
    }

    fn neg(&mut self, x: &impl Variable) -> CircuitVariable {
        let res = self._allocate_local_variable();

        self._append_operation(OpCode::Neg, vec![x.ty()], vec![res.clone()]);

        res
    }

    fn sub_multi(
        &mut self,
        x1: &impl Variable,
        x2: &impl Variable,
        xn: &[&dyn Variable],
    ) -> CircuitVariable {
        let res = self._allocate_local_variable();

        self._append_operation(
            OpCode::Sub,
            get_variable_type_2n(x1, x2, xn),
            vec![res.clone()],
        );

        res
    }

    fn mul_multi(
        &mut self,
        x1: &impl Variable,
        x2: &impl Variable,
        xn: &[&dyn Variable],
    ) -> CircuitVariable {
        let res = self._allocate_local_variable();

        self._append_operation(
            OpCode::Mul,
            get_variable_type_2n(x1, x2, xn),
            vec![res.clone()],
        );

        res
    }

    fn div_unchecked(&mut self, x1: &impl Variable, x2: &impl Variable) -> CircuitVariable {
        let res = self._allocate_local_variable();

        self._append_operation(
            OpCode::DivUnchecked,
            vec![x1.ty(), x2.ty()],
            vec![res.clone()],
        );

        res
    }

    fn div(&mut self, x1: &impl Variable, x2: &impl Variable) -> CircuitVariable {
        let res = self._allocate_local_variable();

        self._append_operation(OpCode::Div, vec![x1.ty(), x2.ty()], vec![res.clone()]);

        res
    }

    fn inverse(&mut self, x: &impl Variable) -> CircuitVariable {
        let res = self._allocate_local_variable();

        self._append_operation(OpCode::Inverse, vec![x.ty()], vec![res.clone()]);

        res
    }

    fn variable_to_binary(&mut self, x: &impl Variable, n: u64) -> Vec<CircuitVariable> {
        let res = self._allocate_local_variable_n(n);

        self._append_operation(OpCode::ToBinary, vec![x.ty(), n.ty()], res.clone());

        res
    }

    fn variable_from_binary(&mut self, b: &[&dyn Variable]) -> CircuitVariable {
        let res = self._allocate_local_variable();

        let inputs = b.iter().map(|x| x.ty()).collect();

        self._append_operation(OpCode::FromBinary, inputs, vec![res.clone()]);

        res
    }

    fn xor(&mut self, x1: &impl Variable, x2: &impl Variable) -> CircuitVariable {
        let res = self._allocate_local_variable();

        self._append_operation(OpCode::Xor, vec![x1.ty(), x2.ty()], vec![res.clone()]);

        res
    }

    fn or(&mut self, x1: &impl Variable, x2: &impl Variable) -> CircuitVariable {
        let res = self._allocate_local_variable();

        self._append_operation(OpCode::Or, vec![x1.ty(), x2.ty()], vec![res.clone()]);

        res
    }

    fn and(&mut self, x1: &impl Variable, x2: &impl Variable) -> CircuitVariable {
        let res = self._allocate_local_variable();

        self._append_operation(OpCode::And, vec![x1.ty(), x2.ty()], vec![res.clone()]);

        res
    }

    fn select(
        &mut self,
        x1: &impl Variable,
        x2: &impl Variable,
        x3: &impl Variable,
    ) -> CircuitVariable {
        let res = self._allocate_local_variable();

        self._append_operation(
            OpCode::Select,
            vec![x1.ty(), x2.ty(), x3.ty()],
            vec![res.clone()],
        );

        res
    }

    fn lookup2(
        &mut self,
        b0: &impl Variable,
        b1: &impl Variable,
        y1: &impl Variable,
        y2: &impl Variable,
        y3: &impl Variable,
        y4: &impl Variable,
    ) -> CircuitVariable {
        let res = self._allocate_local_variable();

        self._append_operation(
            OpCode::Lookup2,
            vec![b0.ty(), b1.ty(), y1.ty(), y2.ty(), y3.ty(), y4.ty()],
            vec![res.clone()],
        );

        res
    }

    fn is_zero(&mut self, x: &impl Variable) -> CircuitVariable {
        let res = self._allocate_local_variable();

        self._append_operation(OpCode::IsZero, vec![x.ty()], vec![res.clone()]);

        res
    }

    fn cmp(&mut self, x1: &impl Variable, x2: &impl Variable) -> CircuitVariable {
        let res = self._allocate_local_variable();

        self._append_operation(OpCode::Cmp, vec![x1.ty(), x2.ty()], vec![res.clone()]);

        res
    }

    fn assert_is_equal(&mut self, x1: &impl Variable, x2: &impl Variable) {
        self._append_operation(OpCode::AssertIsEqual, vec![x1.ty(), x2.ty()], vec![]);
    }

    fn assert_is_different(&mut self, x1: &impl Variable, x2: &impl Variable) {
        self._append_operation(OpCode::AssertIsDifferent, vec![x1.ty(), x2.ty()], vec![]);
    }

    fn assert_is_boolean(&mut self, x: &impl Variable) {
        self._append_operation(OpCode::AssertIsBoolean, vec![x.ty()], vec![]);
    }

    fn assert_is_crumb(&mut self, x: &impl Variable) {
        self._append_operation(OpCode::AssertIsCrumb, vec![x.ty()], vec![]);
    }

    fn assert_is_less_or_equal(&mut self, v: &impl Variable, bound: &impl Variable) {
        self._append_operation(
            OpCode::AssertIsLessOrEqual,
            vec![v.ty(), bound.ty()],
            vec![],
        );
    }

    fn println(&mut self, message: &impl Variable) {
        self._append_operation(OpCode::Println, vec![message.ty()], vec![]);
    }
}

fn get_variable_type_2n(
    x1: &impl Variable,
    x2: &impl Variable,
    xn: &[&dyn Variable],
) -> Vec<VariableType> {
    let mut types = Vec::with_capacity(2 + xn.len());
    types.push(x1.ty());
    types.push(x2.ty());
    for x in xn {
        types.push(x.ty());
    }
    types
}
