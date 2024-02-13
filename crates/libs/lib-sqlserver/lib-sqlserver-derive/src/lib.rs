use proc_macro2::{Ident, TokenStream};
use quote::{quote, quote_spanned, ToTokens};
use syn::spanned::Spanned;
use syn::{Data, DeriveInput, Fields, Type};

#[proc_macro_derive(TryFromRow)]
pub fn try_from_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_try_from(&ast).into()
}

fn impl_try_from(ast: &DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let body = fn_try_from(&ast.data, name);
    let gen = quote! {
        impl TryFromRow<tiberius::Row> for #name {
            fn try_from_row(row: tiberius::Row) -> crate::model::Result<#name> {
                #body
            }
        }
    };
    gen
}

fn fn_try_from(data: &Data, struct_name: &Ident) -> TokenStream {
    match *data {
        Data::Struct(ref data) => match data.fields {
            Fields::Named(ref fields) => {
                let recurse = fields.named.iter().map(|f| {
                    let name = &f.ident;
                    let ty = &f.ty;

                    let is_string = validate_sqlserver_type(ty);

                    if is_string {
                        return quote_spanned! {
                            f.span() => #name: row.try_get(stringify!(#name))?.map(str::to_string),

                        };
                    } else {
                        return quote_spanned! {
                            f.span() => #name: row.try_get(stringify!(#name))?,
                        };
                    }
                });
                quote! {
                    Ok(#struct_name {
                        #(#recurse)*
                    })
                }
            }
            Fields::Unnamed(_) => unimplemented!(),
            Fields::Unit => unimplemented!(),
        },
        Data::Enum(_) | Data::Union(_) => unimplemented!(),
    }
}

fn validate_sqlserver_type(ty: &Type) -> bool {
    let is_string = match ty {
        Type::Path(type_path)
            if type_path.clone().into_token_stream().to_string() == "Option < String >" =>
        {
            true
        }
        _ => false,
    };

    is_string
}
