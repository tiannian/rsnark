use crate::{LocalVariable, Variable};

pub trait API {
    /// returns res = x1 + x2
    fn add(&mut self, x1: &impl Variable, x2: &impl Variable) -> LocalVariable {
        self.add_multi(x1, x2, &[])
    }

    /// returns res = x1+x2+...xn
    fn add_multi(
        &mut self,
        x1: &impl Variable,
        x2: &impl Variable,
        xn: &[&dyn Variable],
    ) -> LocalVariable;

    /// `mul_acc` sets and return a = a + (b*c).
    fn mul_acc(&mut self, a: &impl Variable, b: &impl Variable, c: &impl Variable)
    -> LocalVariable;

    /// returns -x
    fn neg(&mut self, x: &impl Variable) -> LocalVariable;

    /// returns res = x1 - x2
    fn sub(&mut self, x1: &impl Variable, x2: &impl Variable) -> LocalVariable {
        self.sub_multi(x1, x2, &[])
    }

    /// returns res = x1 - x2 - ...xn
    fn sub_multi(
        &mut self,
        x1: &impl Variable,
        x2: &impl Variable,
        xn: &[&dyn Variable],
    ) -> LocalVariable;

    /// returns res = x1 * x2
    fn mul(&mut self, x1: &impl Variable, x2: &impl Variable) -> LocalVariable {
        self.mul_multi(x1, x2, &[])
    }

    /// returns res = x1 * x2 * ...xn
    fn mul_multi(
        &mut self,
        x1: &impl Variable,
        x2: &impl Variable,
        xn: &[&dyn Variable],
    ) -> LocalVariable;

    /// returns x1 / x2
    ///
    /// If x1 == x2 == 0, the return value (0)
    fn div_unchecked(&mut self, x1: &impl Variable, x2: &impl Variable) -> LocalVariable;

    /// returns x1 / x2
    ///
    /// If x2 == 0 the constraint will not be satisfied.
    fn div(&mut self, x1: &impl Variable, x2: &impl Variable) -> LocalVariable;

    /// returns 1 / x
    ///
    /// If x == 0 the constraint will not be satisfied.
    fn inverse(&mut self, x: &impl Variable) -> LocalVariable;

    /// returns x.to_binary()
    ///
    /// n is the number of bits to select (starting from lsb)
    fn to_binary(&mut self, x: &impl Variable, n: u64) -> Vec<LocalVariable>;

    /// packs b in lsb
    fn from_binary(&mut self, b: &[&dyn Variable]) -> LocalVariable;

    /// returns x1 ^ x2
    fn xor(&mut self, x1: &impl Variable, x2: &impl Variable) -> LocalVariable;

    /// returns x1 | x2
    fn or(&mut self, x1: &impl Variable, x2: &impl Variable) -> LocalVariable;

    /// returns x1 & x2
    fn and(&mut self, x1: &impl Variable, x2: &impl Variable) -> LocalVariable;

    /// returns x1 ? x2 : x3
    fn select(
        &mut self,
        x1: &impl Variable,
        x2: &impl Variable,
        x3: &impl Variable,
    ) -> LocalVariable;

    /// performs a 2-bit lookup between y1, y2, y3, y4 based on bits b0, b1
    fn lookup2(
        &mut self,
        b0: &impl Variable,
        b1: &impl Variable,
        y1: &impl Variable,
        y2: &impl Variable,
        y3: &impl Variable,
        y4: &impl Variable,
    ) -> LocalVariable;

    /// returns 1 if x is zero, 0 otherwise
    fn is_zero(&mut self, x: &impl Variable) -> LocalVariable;

    /// compares x1 and x2
    ///
    /// * 1 if i1>i2,
    /// * 0 if i1=i2,
    /// * -1 if i1<i2.
    fn cmp(&mut self, x1: &impl Variable, x2: &impl Variable) -> LocalVariable;

    /// asserts that x1 == x2
    fn assert_is_equal(&mut self, x1: &impl Variable, x2: &impl Variable);

    /// asserts that x1 != x2
    fn assert_is_different(&mut self, x1: &impl Variable, x2: &impl Variable);

    /// asserts that x is a boolean
    fn assert_is_boolean(&mut self, x: &impl Variable);

    /// asserts that x is a crumb (0, 1, 2, 3)
    fn assert_is_crumb(&mut self, x: &impl Variable);

    /// asserts fails if v > bound
    fn assert_is_less_or_equal(&mut self, v: &impl Variable, bound: &impl Variable);

    /// prints a message
    fn println(&mut self, message: &impl Variable);
}
