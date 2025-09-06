use ruint::aliases::U256;

use crate::types::VariableType;

pub trait Variable {
    fn ty(&self) -> VariableType;
}

macro_rules! define_variable {
    ($t:ident, $ty:ident) => {
        #[derive(Debug, Clone)]
        pub struct $t {
            pub(crate) index: u64,
        }

        impl Variable for $t {
            fn ty(&self) -> VariableType {
                VariableType::$ty(self.index)
            }
        }
    };
}

define_variable!(PublicVariable, Public);
define_variable!(PrivateVariable, Private);
define_variable!(LocalVariable, Local);

macro_rules! define_variable_for_from_u256 {
    ($t:ident) => {
        impl Variable for $t {
            fn ty(&self) -> VariableType {
                let x = U256::from(*self);
                VariableType::Constant(x)
            }
        }
    };
}

define_variable_for_from_u256!(U256);
define_variable_for_from_u256!(u128);
define_variable_for_from_u256!(u64);
define_variable_for_from_u256!(u32);
define_variable_for_from_u256!(u16);
define_variable_for_from_u256!(u8);
define_variable_for_from_u256!(i128);
define_variable_for_from_u256!(i64);
define_variable_for_from_u256!(i32);
define_variable_for_from_u256!(i16);
define_variable_for_from_u256!(i8);
define_variable_for_from_u256!(bool);
