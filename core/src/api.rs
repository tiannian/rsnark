use crate::{LocalVariable, Variable};

pub trait API {
    /// returns res = x1+x2+...xn
    fn add(&mut self, x1: &dyn Variable, x2: &dyn Variable, xn: &[&dyn Variable]) -> LocalVariable;

    /// `mul_acc` sets and return a = a + (b*c).
    fn mul_acc(&mut self, a: &dyn Variable, b: &dyn Variable, c: &dyn Variable) -> LocalVariable;

    /// returns -x
    fn neg(&mut self, x: &dyn Variable) -> LocalVariable;

    /// returns res = x1 - x2 - ...xn
    fn sub(&mut self, x1: &dyn Variable, x2: &dyn Variable, xn: &[&dyn Variable]) -> LocalVariable;

    /// returns res = x1 * x2 * ...xn
    fn mul(&mut self, x1: &dyn Variable, x2: &dyn Variable, xn: &[&dyn Variable]) -> LocalVariable;

    /// returns x1 / x2
    ///
    /// If x1 == x2 == 0, the return value (0)
    fn div_unchecked(&mut self, x1: &dyn Variable, x2: &dyn Variable) -> LocalVariable;

    /// returns x1 / x2
    ///
    /// If x2 == 0 the constraint will not be satisfied.
    fn div(&mut self, x1: &dyn Variable, x2: &dyn Variable) -> LocalVariable;

    /// returns 1 / x
    ///
    /// If x == 0 the constraint will not be satisfied.
    fn inverse(&mut self, x: &dyn Variable) -> LocalVariable;

    /// returns x.to_binary()
    ///
    /// n is the number of bits to select (starting from lsb)
    fn to_binary(&mut self, x: &dyn Variable, n: u64) -> Vec<LocalVariable>;

    /// packs b in lsb
    fn from_binary(&mut self, b: &[&dyn Variable]) -> LocalVariable;

    /// returns x1 ^ x2
    fn xor(&mut self, x1: &dyn Variable, x2: &dyn Variable) -> LocalVariable;

    /// returns x1 | x2
    fn or(&mut self, x1: &dyn Variable, x2: &dyn Variable) -> LocalVariable;

    /// returns x1 & x2
    fn and(&mut self, x1: &dyn Variable, x2: &dyn Variable) -> LocalVariable;

    /// returns x1 ? x2 : x3
    fn select(&mut self, x1: &dyn Variable, x2: &dyn Variable, x3: &dyn Variable) -> LocalVariable;

    /// performs a 2-bit lookup between y1, y2, y3, y4 based on bits b0, b1
    fn lookup2(
        &mut self,
        b0: &dyn Variable,
        b1: &dyn Variable,
        y1: &dyn Variable,
        y2: &dyn Variable,
        y3: &dyn Variable,
        y4: &dyn Variable,
    ) -> LocalVariable;

    /// returns 1 if x is zero, 0 otherwise
    fn is_zero(&mut self, x: &dyn Variable) -> LocalVariable;

    /// compares x1 and x2
    ///
    /// * 1 if i1>i2,
    /// * 0 if i1=i2,
    /// * -1 if i1<i2.
    fn cmp(&mut self, x1: &dyn Variable, x2: &dyn Variable) -> LocalVariable;

    /// asserts that x1 == x2
    fn assert_is_equal(&mut self, x1: &dyn Variable, x2: &dyn Variable);

    /// asserts that x1 != x2
    fn assert_is_different(&mut self, x1: &dyn Variable, x2: &dyn Variable);

    /// asserts that x is a boolean
    fn assert_is_boolean(&mut self, x: &dyn Variable);

    /// asserts that x is a crumb (0, 1, 2, 3)
    fn assert_is_crumb(&mut self, x: &dyn Variable);

    /// asserts fails if v > bound
    fn assert_is_less_or_equal(&mut self, v: &dyn Variable, bound: &dyn Variable);

    /// prints a message
    fn println(&mut self, message: &dyn Variable);
}
