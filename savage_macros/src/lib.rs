// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2021  Philipp Emanuel Weidmann <pew@worldwidemann.com>

use darling::FromMeta;
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{
    parse_macro_input, punctuated::Punctuated, token::Comma, AttributeArgs, ExprArray, FnArg,
    ItemFn, Path, Type,
};

#[derive(FromMeta)]
struct Arguments {
    name: String,
    description: String,
    examples: ExprArray,
    categories: ExprArray,
}

/// Generates code required for the marked function to be usable in a function expression.
/// Function metadata is generated from the provided attribute arguments.
#[proc_macro_attribute]
pub fn function(attr: TokenStream, item: TokenStream) -> TokenStream {
    let arguments = match Arguments::from_list(&parse_macro_input!(attr as AttributeArgs)) {
        Ok(arguments) => arguments,
        Err(error) => return TokenStream::from(error.write_errors()),
    };

    let name_argument = arguments.name;
    let description_argument = arguments.description;
    let examples_argument = arguments.examples;
    let categories_argument = arguments.categories;

    let item_fn = parse_macro_input!(item as ItemFn);

    let name = &item_fn.sig.ident;
    let metadata_name = format_ident!("{}_METADATA", name.to_string().to_uppercase());
    let proxy_name = format_ident!("{}_proxy", name);

    let parameters = item_fn.sig.inputs.iter().map(|fn_arg| {
        if let FnArg::Typed(pat_type) = fn_arg {
            if let Type::Path(type_path) = &*pat_type.ty {
                match type_path.path.get_ident().unwrap().to_string().as_str() {
                    "Expression" => quote! { crate::functions::Parameter::Expression },
                    "Integer" => quote! { crate::functions::Parameter::Integer },
                    "Rational" => quote! { crate::functions::Parameter::Rational },
                    "Complex" => quote! { crate::functions::Parameter::Complex },
                    "Vector" => quote! { crate::functions::Parameter::Vector },
                    "Matrix" => quote! { crate::functions::Parameter::Matrix },
                    "bool" => quote! { crate::functions::Parameter::Boolean },
                    _ => unimplemented!(),
                }
            } else {
                unreachable!();
            }
        } else {
            unreachable!();
        }
    });

    let arguments =
        (0..item_fn.sig.inputs.len()).map(|i| quote! { arguments[#i].clone().try_into()? });

    quote! {
        #item_fn

        pub(crate) const #metadata_name: crate::functions::Metadata = crate::functions::Metadata {
            name: #name_argument,
            description: #description_argument,
            parameters: &[#(#parameters),*],
            examples: &#examples_argument,
            categories: &#categories_argument,
        };

        pub(crate) fn #proxy_name(arguments: &[crate::expression::Expression]) ->
            ::std::result::Result<crate::expression::Expression, crate::expression::Expression> {
            ::std::result::Result::Ok(#name(#(#arguments),*).into())
        }
    }
    .into()
}

/// Returns a vector of function definitions generated from the base function paths provided as arguments.
#[proc_macro]
pub fn functions(input: TokenStream) -> TokenStream {
    let mut statements = Vec::new();

    for path in parse_macro_input!(input with Punctuated::<Path, Comma>::parse_terminated) {
        let name = &path.segments.last().unwrap().ident;

        let mut metadata_path = path.clone();
        metadata_path.segments.last_mut().unwrap().ident =
            format_ident!("{}_METADATA", name.to_string().to_uppercase());

        let mut proxy_path = path.clone();
        proxy_path.segments.last_mut().unwrap().ident = format_ident!("{}_proxy", name);

        statements.push(quote! {
            functions.push(Function {
                metadata: #metadata_path,
                implementation: wrap_proxy(#metadata_path.parameters, #proxy_path),
            });
        });
    }

    quote! {{
        let mut functions = Vec::new();

        #(#statements)*

        functions
    }}
    .into()
}
