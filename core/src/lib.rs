//! Core library to write circuits.

pub mod types;

pub mod variable;
#[doc(inline)]
pub use variable::Variable;

mod api;
pub use api::*;

mod builder;
pub use builder::*;

mod circuit;
pub use circuit::*;

mod initer;
pub use initer::*;

mod metadata;
pub use metadata::*;

pub use num::BigInt;

pub use rsnark_macros::Circuit;
