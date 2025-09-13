use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{Data, DeriveInput, Fields, Visibility, parse_macro_input};

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
#[proc_macro_derive(Circuit)]
pub fn circuit_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    match generate_circuit_impl(&input) {
        Ok(tokens) => tokens,
        Err(err) => err.to_compile_error().into(),
    }
}

fn generate_circuit_impl(input: &DeriveInput) -> syn::Result<TokenStream> {
    let name = &input.ident;
    let name_str = name.to_string().to_lowercase();

    // Generate module name: __rsnark_generated_{name}
    let module_name = format_ident!("__rsnark_generated_{}", name_str);

    // Generate CircuitDefine struct name: {Name}CircuitDefine
    let define_name = format_ident!("{}CircuitDefine", name);

    // Generate PublicWitness struct name: {Name}PublicWitness
    let public_witness_name = format_ident!("{}PublicWitness", name);

    // Parse struct fields
    let fields = match &input.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => &fields.named,
            _ => {
                return Err(syn::Error::new_spanned(
                    input,
                    "Only named fields are supported",
                ));
            }
        },
        _ => return Err(syn::Error::new_spanned(input, "Only structs are supported")),
    };

    // Separate public and private fields
    let mut private_fields = Vec::new();
    let mut public_fields = Vec::new();

    for field in fields {
        let field_name = field.ident.as_ref().unwrap();
        let field_type = &field.ty;

        match &field.vis {
            Visibility::Public(_) => {
                public_fields.push((field_name, field_type));
            }
            _ => {
                private_fields.push((field_name, field_type));
            }
        }
    }

    // Generate fields for CircuitDefine struct
    let define_fields = fields.iter().map(|field| {
        let field_name = field.ident.as_ref().unwrap();
        let field_type = &field.ty;

        match &field.vis {
            Visibility::Public(_) => {
                quote! {
                    pub #field_name: ::rsnark_core::PublicCircuitElement<#field_type>
                }
            }
            _ => {
                quote! {
                    pub #field_name: ::rsnark_core::PrivateCircuitElement<#field_type>
                }
            }
        }
    });

    // Generate field initialization in new method
    let new_field_inits = fields.iter().map(|field| {
        let field_name = field.ident.as_ref().unwrap();
        let field_type = &field.ty;

        match &field.vis {
            Visibility::Public(_) => {
                quote! {
                    let #field_name = #field_type::create_public(initer, is_private);
                }
            }
            _ => {
                quote! {
                    let #field_name = #field_type::create_private(initer);
                }
            }
        }
    });

    let field_names: Vec<_> = fields.iter().map(|f| f.ident.as_ref().unwrap()).collect();

    // Generate append_public method implementation for original struct
    let append_public_impl_orig = public_fields.iter().map(|(field_name, _)| {
        quote! {
            self.#field_name.append_public_witness(witness, false);
        }
    });

    // Generate append_public method implementation for PublicWitness struct
    let append_public_impl_witness = public_fields.iter().map(|(field_name, _)| {
        quote! {
            self.#field_name.append_public_witness(witness, false);
        }
    });

    // Generate append_witness method implementation for all fields
    let append_witness_impl = fields.iter().map(|field| {
        let field_name = field.ident.as_ref().unwrap();

        match &field.vis {
            Visibility::Public(_) => {
                // Public fields: is_private = false
                quote! {
                    self.#field_name.append_witness(public, private, false || _is_private);
                }
            }
            _ => {
                // Private fields: is_private = true
                quote! {
                    self.#field_name.append_witness(public, private, true);
                }
            }
        }
    });

    // Generate public witness fields for into_public_witness method
    let public_witness_fields = public_fields.iter().map(|(field_name, _)| {
        quote! {
            #field_name: self.#field_name.into_public_witness()
        }
    });

    // Generate PublicWitness struct fields
    let public_witness_struct_fields = public_fields.iter().map(|(field_name, field_type)| {
        quote! {
            pub #field_name: ::rsnark_core::PublicWitness<#field_type>
        }
    });

    let expanded = quote! {
        mod #module_name {
            use super::*;

            use ::rsnark_core::{
                CircuitWitness, CircuitPublicWitness, BigInt, VariableIniter,
            };

            impl CircuitWitness for #name {
                type PrivateElement = #define_name;
                type PublicElement = #define_name;
                type PublicWitness = #public_witness_name;

                fn create_public(initer: &mut VariableIniter, is_private: bool) -> Self::PublicElement {
                    #define_name::new(initer, is_private)
                }

                fn create_private(initer: &mut VariableIniter) -> Self::PrivateElement {
                    #define_name::new(initer, true)
                }

                fn append_witness(&self, public: &mut Vec<BigInt>, private: &mut Vec<BigInt>, _is_private: bool) {
                    #(#append_witness_impl)*
                }

                fn into_public_witness(self) -> Self::PublicWitness {
                    #public_witness_name {
                        #(#public_witness_fields,)*
                    }
                }
            }

            #[doc(hidden)]
            pub struct #define_name {
                #(#define_fields,)*
            }

            impl #define_name {
                fn new(initer: &mut VariableIniter, is_private: bool) -> Self {
                    #(#new_field_inits)*

                    Self {
                        #(#field_names,)*
                    }
                }
            }

            impl CircuitPublicWitness for #name {
                fn append_public_witness(&self, witness: &mut Vec<BigInt>, _is_private: bool) {
                    #(#append_public_impl_orig)*
                }
            }

            #[doc(hidden)]
            pub struct #public_witness_name {
                #(#public_witness_struct_fields,)*
            }

            impl CircuitPublicWitness for #public_witness_name {
                fn append_public_witness(&self, witness: &mut Vec<BigInt>, _is_private: bool) {
                    #(#append_public_impl_witness)*
                }
            }
        }
    };

    Ok(TokenStream::from(expanded))
}
