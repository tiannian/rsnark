use crate::{
    Metadata, Variable,
    types::{OpCode, VariableType},
};

/// The main API trait for building arithmetic circuits in zero-knowledge proof systems.
///
/// This trait provides a comprehensive set of operations for constructing arithmetic circuits,
/// including basic arithmetic operations, logical operations, assertions, and utility functions.
/// All operations work with variables that implement the `Variable` trait, enabling flexible
/// circuit construction with different variable types.
pub trait API {
    fn metadata(&self) -> &impl Metadata;

    fn append_operation(
        &mut self,
        op: OpCode,
        inputs: Vec<VariableType>,
        outputs: Vec<VariableType>,
    );

    fn allocate_local_variable(&mut self) -> VariableType;

    fn allocate_local_variable_n(&mut self, n: u64) -> Vec<VariableType> {
        let mut res = Vec::with_capacity(n as usize);
        for _ in 0..n {
            res.push(self.allocate_local_variable());
        }
        res
    }

    /// Performs addition of two variables: res = x1 + x2
    ///
    /// This is a convenience method that calls `add_multi` with no additional variables.
    ///
    /// # Arguments
    /// * `x1` - First operand
    /// * `x2` - Second operand
    ///
    /// # Returns
    /// A new local variable containing the sum
    fn add(&mut self, x1: &impl Variable, x2: &impl Variable) -> VariableType {
        self.add_multi(x1, x2, &[])
    }

    /// Performs addition of multiple variables: res = x1 + x2 + ... + xn
    ///
    /// # Arguments
    /// * `x1` - First operand
    /// * `x2` - Second operand  
    /// * `xn` - Additional operands to sum
    ///
    /// # Returns
    /// A new local variable containing the sum of all operands
    fn add_multi(
        &mut self,
        x1: &impl Variable,
        x2: &impl Variable,
        xn: &[&dyn Variable],
    ) -> VariableType {
        let res = self.allocate_local_variable();

        self.append_operation(
            OpCode::Add,
            get_variable_type_2n(x1, x2, xn),
            vec![res.clone()],
        );

        res
    }

    /// Performs multiply-accumulate operation: res = a + (b * c)
    ///
    /// This is an optimized operation that combines multiplication and addition
    /// in a single constraint, which can be more efficient than separate operations.
    ///
    /// # Arguments
    /// * `a` - The accumulator value
    /// * `b` - First multiplicand
    /// * `c` - Second multiplicand
    ///
    /// # Returns
    /// A new local variable containing the result a + (b * c)
    fn mul_acc(&mut self, a: &impl Variable, b: &impl Variable, c: &impl Variable) -> VariableType {
        let res = self.allocate_local_variable();

        self.append_operation(
            OpCode::MulAcc,
            vec![a.ty(), b.ty(), c.ty()],
            vec![res.clone()],
        );

        res
    }

    /// Performs negation: res = -x
    ///
    /// # Arguments
    /// * `x` - The variable to negate
    ///
    /// # Returns
    /// A new local variable containing the negated value
    fn neg(&mut self, x: &impl Variable) -> VariableType {
        let res = self.allocate_local_variable();

        self.append_operation(OpCode::Neg, vec![x.ty()], vec![res.clone()]);

        res
    }

    /// Performs subtraction of two variables: res = x1 - x2
    ///
    /// This is a convenience method that calls `sub_multi` with no additional variables.
    ///
    /// # Arguments
    /// * `x1` - Minuend (value to subtract from)
    /// * `x2` - Subtrahend (value to subtract)
    ///
    /// # Returns
    /// A new local variable containing the difference
    fn sub(&mut self, x1: &impl Variable, x2: &impl Variable) -> VariableType {
        self.sub_multi(x1, x2, &[])
    }

    /// Performs subtraction of multiple variables: res = x1 - x2 - ... - xn
    ///
    /// # Arguments
    /// * `x1` - Minuend (value to subtract from)
    /// * `x2` - First subtrahend
    /// * `xn` - Additional values to subtract
    ///
    /// # Returns
    /// A new local variable containing the result of all subtractions
    fn sub_multi(
        &mut self,
        x1: &impl Variable,
        x2: &impl Variable,
        xn: &[&dyn Variable],
    ) -> VariableType {
        let res = self.allocate_local_variable();

        self.append_operation(
            OpCode::Sub,
            get_variable_type_2n(x1, x2, xn),
            vec![res.clone()],
        );

        res
    }

    /// Performs multiplication of two variables: res = x1 * x2
    ///
    /// This is a convenience method that calls `mul_multi` with no additional variables.
    ///
    /// # Arguments
    /// * `x1` - First multiplicand
    /// * `x2` - Second multiplicand
    ///
    /// # Returns
    /// A new local variable containing the product
    fn mul(&mut self, x1: &impl Variable, x2: &impl Variable) -> VariableType {
        self.mul_multi(x1, x2, &[])
    }

    /// Performs multiplication of multiple variables: res = x1 * x2 * ... * xn
    ///
    /// # Arguments
    /// * `x1` - First multiplicand
    /// * `x2` - Second multiplicand
    /// * `xn` - Additional multiplicands
    ///
    /// # Returns
    /// A new local variable containing the product of all operands
    fn mul_multi(
        &mut self,
        x1: &impl Variable,
        x2: &impl Variable,
        xn: &[&dyn Variable],
    ) -> VariableType {
        let res = self.allocate_local_variable();

        self.append_operation(
            OpCode::Mul,
            get_variable_type_2n(x1, x2, xn),
            vec![res.clone()],
        );

        res
    }

    /// Performs unchecked division: res = x1 / x2
    ///
    /// This division operation does not enforce that the divisor is non-zero.
    /// If both x1 and x2 are zero, the result is defined as 0.
    ///
    /// # Arguments
    /// * `x1` - Dividend
    /// * `x2` - Divisor
    ///
    /// # Returns
    /// A new local variable containing the quotient
    ///
    /// # Safety
    /// This operation does not verify that x2 ≠ 0. Use `div` for checked division.
    fn div_unchecked(&mut self, x1: &impl Variable, x2: &impl Variable) -> VariableType {
        let res = self.allocate_local_variable();

        self.append_operation(
            OpCode::DivUnchecked,
            vec![x1.ty(), x2.ty()],
            vec![res.clone()],
        );

        res
    }

    /// Performs checked division: res = x1 / x2
    ///
    /// This division operation enforces that the divisor is non-zero.
    /// The circuit constraint will fail if x2 equals zero.
    ///
    /// # Arguments
    /// * `x1` - Dividend
    /// * `x2` - Divisor (must be non-zero)
    ///
    /// # Returns
    /// A new local variable containing the quotient
    ///
    /// # Panics
    /// The circuit will be unsatisfiable if x2 == 0
    fn div(&mut self, x1: &impl Variable, x2: &impl Variable) -> VariableType {
        let res = self.allocate_local_variable();

        self.append_operation(OpCode::Div, vec![x1.ty(), x2.ty()], vec![res.clone()]);

        res
    }

    /// Computes the multiplicative inverse: res = 1 / x
    ///
    /// This operation enforces that x is non-zero. The circuit constraint
    /// will fail if x equals zero.
    ///
    /// # Arguments
    /// * `x` - The variable to invert (must be non-zero)
    ///
    /// # Returns
    /// A new local variable containing the multiplicative inverse
    ///
    /// # Panics
    /// The circuit will be unsatisfiable if x == 0
    fn inverse(&mut self, x: &impl Variable) -> VariableType {
        let res = self.allocate_local_variable();

        self.append_operation(OpCode::Inverse, vec![x.ty()], vec![res.clone()]);

        res
    }

    /// Converts a variable to its binary representation
    ///
    /// Decomposes the input variable into its constituent bits, returning
    /// them as a vector of boolean variables.
    ///
    /// # Arguments
    /// * `x` - The variable to decompose
    /// * `n` - Number of bits to extract (starting from least significant bit)
    ///
    /// # Returns
    /// A vector of local variables representing the binary decomposition,
    /// where index 0 is the least significant bit
    fn variable_to_binary(&mut self, x: &impl Variable, n: u64) -> Vec<VariableType> {
        let res = self.allocate_local_variable_n(n);

        self.append_operation(OpCode::ToBinary, vec![x.ty(), n.ty()], res.clone());

        res
    }

    /// Reconstructs a variable from its binary representation
    ///
    /// Combines a vector of bit variables into a single field element,
    /// with the first element being the least significant bit.
    ///
    /// # Arguments
    /// * `b` - Array of bit variables (each should be 0 or 1)
    ///
    /// # Returns
    /// A local variable representing the packed binary value
    fn variable_from_binary(&mut self, b: &[&dyn Variable]) -> VariableType {
        let res = self.allocate_local_variable();

        self.append_operation(
            OpCode::FromBinary,
            b.iter().map(|x| x.ty()).collect(),
            vec![res.clone()],
        );

        res
    }

    /// Performs bitwise XOR operation: res = x1 ^ x2
    ///
    /// # Arguments
    /// * `x1` - First operand
    /// * `x2` - Second operand
    ///
    /// # Returns
    /// A new local variable containing the XOR result
    fn xor(&mut self, x1: &impl Variable, x2: &impl Variable) -> VariableType {
        let res = self.allocate_local_variable();

        self.append_operation(OpCode::Xor, vec![x1.ty(), x2.ty()], vec![res.clone()]);

        res
    }

    /// Performs bitwise OR operation: res = x1 | x2
    ///
    /// # Arguments
    /// * `x1` - First operand
    /// * `x2` - Second operand
    ///
    /// # Returns
    /// A new local variable containing the OR result
    fn or(&mut self, x1: &impl Variable, x2: &impl Variable) -> VariableType {
        let res = self.allocate_local_variable();

        self.append_operation(OpCode::Or, vec![x1.ty(), x2.ty()], vec![res.clone()]);

        res
    }

    /// Performs bitwise AND operation: res = x1 & x2
    ///
    /// # Arguments
    /// * `x1` - First operand
    /// * `x2` - Second operand
    ///
    /// # Returns
    /// A new local variable containing the AND result
    fn and(&mut self, x1: &impl Variable, x2: &impl Variable) -> VariableType {
        let res = self.allocate_local_variable();

        self.append_operation(OpCode::And, vec![x1.ty(), x2.ty()], vec![res.clone()]);

        res
    }

    /// Performs conditional selection: res = x1 ? x2 : x3
    ///
    /// If x1 is non-zero (true), returns x2; otherwise returns x3.
    /// This is equivalent to a ternary operator in programming languages.
    ///
    /// # Arguments
    /// * `x1` - Condition variable (typically 0 or 1)
    /// * `x2` - Value to return if condition is true
    /// * `x3` - Value to return if condition is false
    ///
    /// # Returns
    /// A new local variable containing the selected value
    fn select(
        &mut self,
        x1: &impl Variable,
        x2: &impl Variable,
        x3: &impl Variable,
    ) -> VariableType {
        let res = self.allocate_local_variable();

        self.append_operation(
            OpCode::Select,
            vec![x1.ty(), x2.ty(), x3.ty()],
            vec![res.clone()],
        );

        res
    }

    /// Performs a 2-bit lookup table operation
    ///
    /// Selects one of four values (y1, y2, y3, y4) based on a 2-bit index
    /// formed by concatenating bits b1 and b0 (b1 is MSB, b0 is LSB).
    ///
    /// # Arguments
    /// * `b0` - Least significant bit of the index
    /// * `b1` - Most significant bit of the index
    /// * `y1` - Value for index 00 (b1=0, b0=0)
    /// * `y2` - Value for index 01 (b1=0, b0=1)
    /// * `y3` - Value for index 10 (b1=1, b0=0)
    /// * `y4` - Value for index 11 (b1=1, b0=1)
    ///
    /// # Returns
    /// The selected value based on the 2-bit index
    fn lookup2(
        &mut self,
        b0: &impl Variable,
        b1: &impl Variable,
        y1: &impl Variable,
        y2: &impl Variable,
        y3: &impl Variable,
        y4: &impl Variable,
    ) -> VariableType {
        let res = self.allocate_local_variable();

        self.append_operation(
            OpCode::Lookup2,
            vec![b0.ty(), b1.ty(), y1.ty(), y2.ty(), y3.ty(), y4.ty()],
            vec![res.clone()],
        );

        res
    }

    /// Tests if a variable is zero
    ///
    /// Returns 1 if the input variable equals zero, 0 otherwise.
    /// This is useful for implementing conditional logic based on zero checks.
    ///
    /// # Arguments
    /// * `x` - The variable to test
    ///
    /// # Returns
    /// A boolean variable (1 if x == 0, 0 if x != 0)
    fn is_zero(&mut self, x: &impl Variable) -> VariableType {
        let res = self.allocate_local_variable();

        self.append_operation(OpCode::IsZero, vec![x.ty()], vec![res.clone()]);

        res
    }

    /// Compares two variables and returns the comparison result
    ///
    /// Returns a three-way comparison result:
    /// * 1 if x1 > x2
    /// * 0 if x1 = x2  
    /// * -1 if x1 < x2
    ///
    /// # Arguments
    /// * `x1` - First value to compare
    /// * `x2` - Second value to compare
    ///
    /// # Returns
    /// A local variable containing the comparison result (-1, 0, or 1)
    fn cmp(&mut self, x1: &impl Variable, x2: &impl Variable) -> VariableType {
        let res = self.allocate_local_variable();

        self.append_operation(OpCode::Cmp, vec![x1.ty(), x2.ty()], vec![res.clone()]);

        res
    }

    /// Asserts that two variables are equal
    ///
    /// Adds a constraint requiring x1 == x2. The circuit will be
    /// unsatisfiable if this condition is not met.
    ///
    /// # Arguments
    /// * `x1` - First variable
    /// * `x2` - Second variable
    ///
    /// # Panics
    /// The circuit will be unsatisfiable if x1 != x2
    fn assert_is_equal(&mut self, x1: &impl Variable, x2: &impl Variable) {
        self.append_operation(OpCode::AssertIsEqual, vec![x1.ty(), x2.ty()], vec![]);
    }

    /// Asserts that two variables are different
    ///
    /// Adds a constraint requiring x1 != x2. The circuit will be
    /// unsatisfiable if this condition is not met.
    ///
    /// # Arguments
    /// * `x1` - First variable
    /// * `x2` - Second variable
    ///
    /// # Panics
    /// The circuit will be unsatisfiable if x1 == x2
    fn assert_is_different(&mut self, x1: &impl Variable, x2: &impl Variable) {
        self.append_operation(OpCode::AssertIsDifferent, vec![x1.ty(), x2.ty()], vec![]);
    }

    /// Asserts that a variable is a boolean value
    ///
    /// Adds a constraint requiring x to be either 0 or 1.
    /// The circuit will be unsatisfiable if x has any other value.
    ///
    /// # Arguments
    /// * `x` - The variable to constrain as boolean
    ///
    /// # Panics
    /// The circuit will be unsatisfiable if x is not 0 or 1
    fn assert_is_boolean(&mut self, x: &impl Variable) {
        self.append_operation(OpCode::AssertIsBoolean, vec![x.ty()], vec![]);
    }

    /// Asserts that a variable is a crumb (2-bit value)
    ///
    /// Adds a constraint requiring x to be one of {0, 1, 2, 3}.
    /// The circuit will be unsatisfiable if x has any other value.
    ///
    /// # Arguments
    /// * `x` - The variable to constrain as a crumb
    ///
    /// # Panics
    /// The circuit will be unsatisfiable if x is not in {0, 1, 2, 3}
    fn assert_is_crumb(&mut self, x: &impl Variable) {
        self.append_operation(OpCode::AssertIsCrumb, vec![x.ty()], vec![]);
    }

    /// Asserts that a variable is less than or equal to a bound
    ///
    /// Adds a constraint requiring v <= bound. The circuit will be
    /// unsatisfiable if this condition is not met.
    ///
    /// # Arguments
    /// * `v` - The variable to check
    /// * `bound` - The upper bound
    ///
    /// # Panics
    /// The circuit will be unsatisfiable if v > bound
    fn assert_is_less_or_equal(&mut self, v: &impl Variable, bound: &impl Variable) {
        self.append_operation(
            OpCode::AssertIsLessOrEqual,
            vec![v.ty(), bound.ty()],
            vec![],
        );
    }

    /// Prints a debug message during circuit execution
    ///
    /// This is primarily useful for debugging circuits. The actual
    /// implementation depends on the backend prover system.
    ///
    /// # Arguments
    /// * `message` - The variable value to print
    fn println(&mut self, message: &impl Variable) {
        self.append_operation(OpCode::Println, vec![message.ty()], vec![]);
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
