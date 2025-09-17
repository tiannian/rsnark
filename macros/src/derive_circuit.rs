use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{ItemStruct, Visibility};

pub fn generate_circuit_impl(input: &ItemStruct) -> syn::Result<TokenStream> {
    let name = &input.ident;
    let name_str = name.to_string().to_lowercase();

    let generics = &input.generics;

    // Generate module name: __rsnark_generated_{name}
    let module_name = format_ident!("__rsnark_generated_{}", name_str);

    // Generate Witness struct name: {Name}Witness
    let witness_name = format_ident!("{}Witness", name);

    // Generate PublicWitness struct name: {Name}PublicWitness
    let public_witness_name = format_ident!("{}PublicWitness", name);

    // Parse struct fields
    let fields = &input.fields;

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

    // Generate modified fields for the original struct
    let modified_struct_fields = fields.iter().map(|field| {
        let field_name = field.ident.as_ref().unwrap();
        let field_type = &field.ty;
        let vis = &field.vis;

        quote! {
            #vis #field_name: <#field_type as ::rsnark_core::CircuitWitness>::CircuitElement
        }
    });

    // Generate fields for Witness struct (using original types)
    let witness_struct_fields = fields.iter().map(|field| {
        let field_name = field.ident.as_ref().unwrap();
        let field_type = &field.ty;

        quote! {
            pub #field_name: #field_type
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
            pub #field_name: <#field_type as ::rsnark_core::CircuitWitness>::PublicWitness
        }
    });

    // Generate where clause for CircuitWitness bounds for generic types
    let where_clause: Vec<_> = generics
        .params
        .iter()
        .filter_map(|param| match param {
            syn::GenericParam::Type(type_param) => {
                let ident = &type_param.ident;
                Some(quote! { #ident: ::rsnark_core::CircuitWitness<CircuitElement = ::rsnark_core::types::VariableType> })
            }
            _ => None,
        })
        .collect();

    // Determine if we have a where clause to add
    let where_clause_tokens = if where_clause.is_empty() {
        quote! {}
    } else {
        quote! { where #(#where_clause,)* }
    };

    let expanded = quote! {
        #[doc(hidden)]
        pub struct #name #generics #where_clause_tokens {
            #(#modified_struct_fields,)*
        }

        mod #module_name {
            use super::*;
            use ::rsnark_core::{BigInt, CircuitPublicWitness, CircuitWitness, VariableIniter};
            use rsnark_core::CircuitElement;

            impl #generics #name #generics #where_clause_tokens {
                fn new(initer: &mut VariableIniter, is_private: bool) -> Self {
                    #(#new_field_inits)*
                    Self {
                        #(#field_names,)*
                    }
                }
            }

            impl #generics CircuitElement for #name #generics #where_clause_tokens {
                type CircuitWitness = #witness_name #generics;
            }

            pub struct #witness_name #generics #where_clause_tokens {
                #(#witness_struct_fields,)*
            }

            impl #generics CircuitWitness for #witness_name #generics #where_clause_tokens {
                type CircuitElement = #name #generics;
                type PublicWitness = #public_witness_name #generics;

                fn create_public(initer: &mut VariableIniter, is_private: bool) -> Self::CircuitElement {
                    #name::new(initer, is_private)
                }

                fn create_private(initer: &mut VariableIniter) -> Self::CircuitElement {
                    #name::new(initer, true)
                }

                fn append_witness(
                    &self,
                    public: &mut Vec<BigInt>,
                    private: &mut Vec<BigInt>,
                    _is_private: bool,
                ) {
                    #(#append_witness_impl)*
                }

                fn into_public_witness(self) -> Self::PublicWitness {
                    #public_witness_name {
                        #(#public_witness_fields,)*
                    }
                }
            }

            impl #generics CircuitPublicWitness for #witness_name #generics #where_clause_tokens {
                fn append_public_witness(&self, witness: &mut Vec<BigInt>, _is_private: bool) {
                    #(#append_public_impl_orig)*
                }
            }

            #[doc(hidden)]
            pub struct #public_witness_name #generics #where_clause_tokens {
                #(#public_witness_struct_fields,)*
            }

            impl #generics CircuitPublicWitness for #public_witness_name #generics #where_clause_tokens {
                fn append_public_witness(&self, witness: &mut Vec<BigInt>, _is_private: bool) {
                    #(#append_public_impl_witness)*
                }
            }
        }
    };

    Ok(TokenStream::from(expanded))
}
