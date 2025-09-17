use proc_macro::TokenStream;
use syn::{ItemStruct, parse_macro_input};

mod derive_circuit;

/// Circuit derive macro
///
/// Automatically generates CircuitElement implementation and related helper code
/// for the annotated struct.
///
/// Usage:
/// ```rust, ignore
/// use rsnark_macros::Circuit;
///
/// #[derive(Circuit)]
/// pub struct MyCircuit {
///     a: u32,        // private field
///     pub b: u32,    // public field
/// }
/// ```
#[proc_macro_attribute]
pub fn circuit(_: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemStruct);

    match derive_circuit::generate_circuit_impl(&input) {
        Ok(tokens) => tokens,
        Err(err) => err.to_compile_error().into(),
    }
}
