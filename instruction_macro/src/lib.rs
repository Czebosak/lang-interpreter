use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{Data, DeriveInput, Fields, Ident, Type, parse_macro_input};

fn helper(ident: &Ident, ty: &Type) -> proc_macro2::TokenStream {
    match ty {
        Type::Reference(tr) => {
            if let Type::Slice(slice) = &*tr.elem {
                if let Type::Path(tp) = &*slice.elem {
                    if tp.path.is_ident("u8") {
                        return quote! { _data.push(#ident.len() as u8); _data.extend_from_slice(#ident); };
                    }
                }
            }
            panic!("Unsupported reference type");
        },
        // Primitive integers and floats
        Type::Path(tp) => {
            let seg = tp.path.segments.last().unwrap().ident.to_string();
            match seg.as_str() {
                "u8" | "u16" | "u32" | "u64" | "i8" | "i16" | "i32" | "i64" | "f32" | "f64" => {
                    quote! { _data.extend(#ident.to_le_bytes()); }
                },
                _ => panic!("Unsupported field type: {}", seg),
            }
        } _ => panic!("Unsupported field type"),
    }
}

#[proc_macro_derive(Instruction)]
pub fn instruction(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let data = match &input.data {
        Data::Enum(data_enum) => data_enum,
        _ => panic!("MyDerive only works on enums"),
    };

    let enum_kind_name = format_ident!("{}Kind", name);
    
    let match_arms = data.variants.iter().map(|variant| {
        let vname = &variant.ident;
        match &variant.fields {
            Fields::Unit => {
                quote! { #name::#vname => _data.push(#enum_kind_name::#vname as u8), }
            },
            Fields::Unnamed(fields) => {
                // Create pattern like: ( _ , _ , _ )
                let pats = fields.unnamed.iter().enumerate().map(|(i, _)| {
                    let ident = format_ident!("f{}", i);
                    quote! { #ident }
                });
                let x = fields.unnamed.iter().map(|f| {
                    helper(f.ident.as_ref().unwrap(), &f.ty)
                });
                quote! { #name::#vname( #( #pats ),* ) => { _data.push(#enum_kind_name::#vname as u8); #( #x )* }, }
            },
            Fields::Named(fields) => {
                // Create pattern like: { a: _, b: _ }
                let pats = fields.named.iter().map(|f| {
                    let fname = &f.ident;
                    quote! { #fname }
                });
                let x = fields.named.iter().map(|f| {
                    helper(f.ident.as_ref().unwrap(), &f.ty)
                });
                quote! { #name::#vname { #( #pats ),* } => { _data.push(#enum_kind_name::#vname as u8); #( #x )* }, }
            }
        } 
    });

    let variants = data.variants.iter().map(|variant| {
        let vname = &variant.ident;
        quote! { #vname, }
    });

    let generics = &input.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl(); 

    // Example: generate an empty impl
    let expanded = quote! {
        #[repr(u8)]
        enum #enum_kind_name {
            #( #variants )*
        }

        impl #impl_generics #name #ty_generics #where_clause {
            pub fn to_bytes(&self) -> Vec<u8> {
                let mut _data = Vec::new();
                match self { #( #match_arms )* }
                _data
            }
        }
    };
    expanded.into()
}
