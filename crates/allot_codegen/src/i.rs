use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{parse_quote, Data, DeriveInput, Expr, Ident};

use crate::structs::LibReturnMacroInput;

pub fn raw_enum(input: DeriveInput) -> TokenStream {
    let fields = match input.data {
        Data::Enum(item) => item.variants,
        _ => panic!("RawEnum only works on enums."),
    };

    // Generate Fields
    let enum_id = input.ident;
    let name = Ident::new(&format!("Raw{}", enum_id), Span::call_site());
    let ids = fields.clone().into_iter().map(|v| {
        let id = v.ident;
        quote! {
            #id,
        }
    });

    // Generate to_raw
    let matches = fields.into_iter().map(|v| {
        let id = v.ident.clone();
        let num_fields = v.fields.len();

        let ignore_fields = if num_fields > 0 {
            let mut s = "(".to_string();
            for _ in 0..(num_fields - 1) {
                s.push_str("_, ");
            }
            s.push_str("_)");
            s
        }
        else {
            String::new()
        };
        let ignore_fields: TokenStream = ignore_fields.parse().unwrap();

        quote! {
            #enum_id::#id #ignore_fields => #name::#id,
        }
    });

    quote! {
        #[automatically_derived]
        impl #enum_id {
            pub fn to_raw(&self) -> #name {
                match self {
                    #(#matches)*
                }
            }
        }

        #[automatically_derived]
        #[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, IntoPrimitive, TryFromPrimitive)]
        #[repr(u8)]
        pub enum #name {
            #(#ids)*
        }
    }
}

pub fn lib_return(input: LibReturnMacroInput) -> TokenStream {
    if input.0.len() > 5 {
        panic!("lib_return macro only allows for up to 5 inputs.");
    }

    let mut vec: Vec<Expr> = vec![parse_quote!(None); 5];
    for (i, e) in input.0.iter().enumerate() {
        vec[i] = parse_quote!(Some(#e));
    }

    let fields = vec.into_iter().map(|v| {
        quote! {
            #v,
        }
    });

    quote! {
        return (#(#fields)*)
    }
}
