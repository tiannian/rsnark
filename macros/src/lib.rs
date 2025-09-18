use proc_macro::TokenStream;
use syn::{ItemStruct, parse_macro_input};

mod derive_circuit;

/// Circuit attribute macro for defining zero-knowledge circuits.
///
/// This macro automatically generates the necessary implementations for circuit structures,
/// including witness handling, variable management, and circuit element traits.
///
/// # Features
///
/// - **Automatic witness generation**: Creates corresponding witness structures
/// - **Public/private field handling**: Respects Rust visibility modifiers
/// - **Generic support**: Works with generic circuit structures
/// - **Type safety**: Ensures proper variable and witness type relationships
///
/// # Usage
///
/// ## Basic Circuit
/// ```rust,ignore
/// use rsnark::circuit;
///
/// #[circuit]
/// pub struct MyCircuit {
///     a: u32,        // private input
///     pub b: u32,    // public input
/// }
/// ```
///
/// ## Generic Circuit
/// ```rust,ignore
/// #[circuit]
/// pub struct GenericCircuit<T> {
///     input: T,
///     pub output: T,
/// }
/// ```
///
/// # Generated Code
///
/// The macro generates:
/// - A modified circuit struct with `CircuitElementInner<T>` fields
/// - A corresponding `{Name}Witness` struct for witness data
/// - A `{Name}PublicWitness` struct for public witness data
/// - Implementations of `CircuitElement`, `CircuitWitness`, and `CircuitPublicWitness` traits
///
/// # Field Visibility
///
/// - Fields **without** `pub` become **private inputs**
/// - Fields **with** `pub` become **public inputs**
///
/// # Requirements
///
/// For generic circuits, type parameters must implement appropriate circuit traits.
/// See the documentation for specific trait bounds required.
#[proc_macro_attribute]
pub fn circuit(_: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemStruct);

    match derive_circuit::generate_circuit_impl(&input) {
        Ok(tokens) => tokens,
        Err(err) => err.to_compile_error().into(),
    }
}
