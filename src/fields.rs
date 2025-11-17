use crate::item::enum_item::EnumInnerFields;
use proc_macro2::Span;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{Ident, LitBool, LitStr, Result, Token, parse::Parse, parse::ParseStream};

#[derive(Clone)]
pub enum BrickFieldArgs {
    ConvertFieldFn(LitStr),
    FnFromExtern(LitStr),
    Rename(LitStr),
    Exclude(LitBool),
    IsFallible(LitBool),
}

impl Parse for BrickFieldArgs {
    fn parse(input: ParseStream) -> Result<Self> {
        let keyword: Ident = input.parse()?;
        let _eq_token: Token![=] = input.parse()?;

        match keyword {
            k if k == "transform_func" => Ok(BrickFieldArgs::ConvertFieldFn(input.parse()?)),
            k if k == "fn_from_extern" => Ok(BrickFieldArgs::FnFromExtern(input.parse()?)),
            k if k == "rename" => Ok(BrickFieldArgs::Rename(input.parse()?)),
            k if k == "exclude" => Ok(BrickFieldArgs::Exclude(input.parse()?)),
            k if k == "is_fallible" => Ok(BrickFieldArgs::IsFallible(input.parse()?)),
            _ => Err(syn::Error::new(keyword.span(), "Attribute not supported")),
        }
    }
}

impl BrickFieldArgs {
    /// Create the struct template which will be used inside the field to map the path src: target
    ///
    /// # Arguments
    /// * `name` - The name of the struct template.
    /// * `fields` - The fields of the struct template.
    pub(crate) fn create_struct_template(name: Ident, fields: Vec<Self>) -> TokenStream {
        let mut from_field_name: Option<Ident> = Some(name.clone());
        let mut fn_from_extern: Option<syn::Ident> = None;
        let mut func: Option<Ident> = None;
        let mut to_skip = false;
        let mut is_fallible = false;

        for field in fields {
            if let Self::Rename(n) = field.to_owned() {
                from_field_name = Some(Ident::new(&n.value(), Span::call_site()));
            }

            if let Self::FnFromExtern(t) = field.to_owned() {
                fn_from_extern = Some(Ident::new(&t.value(), Span::call_site()));
            }

            if let Self::ConvertFieldFn(f) = field.to_owned() {
                func = Some(Ident::new(&f.value(), Span::call_site()));
            }

            if let Self::IsFallible(r) = field.to_owned() {
                is_fallible = r.value();
            }

            // In the case where we exclude the field, we just skip to output that field.
            if let Self::Exclude(e) = field.to_owned()
                && e.value()
            {
                to_skip = true;
            }
        }

        let res_call = match is_fallible {
            true => quote! {
                (arg.#from_field_name)?
            },
            false => quote! {
                (arg.#from_field_name)
            },
        };

        match to_skip {
            true => quote! {
                #name: Default::default()
            },
            false => match func {
                Some(f) => match fn_from_extern {
                    Some(t) => quote! {
                        #name: #t::#f #res_call
                    },
                    None => quote! {
                        #name: #f #res_call
                    },
                },
                None => quote! { #name: arg.#from_field_name },
            },
        }
    }

    /// Create the enum template which will be used inside the field to map the path src: target within a match statement.
    /// This will create an enum value for "each statement" e.g:
    ///    - Source::Foo => Target::Foo
    ///
    /// # Arguments
    /// * `name` - The name of the enum template.
    /// * `source` - The source of the enum template.
    /// * `fields` - The fields of the enum template.
    pub fn create_enum_template(
        name: Ident,
        source: Option<Ident>,
        fields: Vec<Self>,
        enum_fields: EnumInnerFields,
    ) -> TokenStream {
        let mut rename: Option<Ident> = Some(name.clone());
        let mut to_skip = false;
        let mut func: Option<Ident> = None;
        let mut fn_from_extern: Option<Ident> = None;

        for field in fields {
            if let Self::Rename(rename_field) = field.to_owned() {
                rename = Some(Ident::new(&rename_field.value(), Span::call_site()));
            }

            if let Self::Exclude(e) = field.to_owned()
                && e.value()
            {
                to_skip = true;
            }

            if let Self::ConvertFieldFn(fn_field) = field.to_owned() {
                func = Some(Ident::new(&fn_field.value(), Span::call_site()));
            }

            if let Self::FnFromExtern(t) = field.to_owned() {
                fn_from_extern = Some(Ident::new(&t.value(), Span::call_site()));
            }
        }

        match to_skip {
            true => quote! {},
            false => match func {
                Some(f) => {
                    enum_builder::generate_enum_fn(source, rename, fn_from_extern, f, &enum_fields)
                }
                None => match enum_fields {
                    EnumInnerFields::Unnamed(unnamed_enum_fields) => {
                        quote! {
                            #source::#rename #unnamed_enum_fields => Self::#name #unnamed_enum_fields
                        }
                    }
                    EnumInnerFields::Named(named_enum_fields) => {
                        quote! {
                            #source::#rename{#named_enum_fields} => Self::#name {#named_enum_fields}
                        }
                    }
                    EnumInnerFields::Unit => {
                        quote! {
                            #source::#rename => Self::#name
                        }
                    }
                },
            },
        }
    }
}

mod enum_builder {
    use super::*;

    pub fn generate_enum_fn(
        source: Option<Ident>,
        rename: Option<Ident>,
        extern_fn: Option<Ident>,
        fn_tmpl: Ident,
        enum_inner_fields: &EnumInnerFields,
    ) -> TokenStream {
        let (match_ident, fn_args) = match enum_inner_fields {
            EnumInnerFields::Unnamed(tk) => (tk.clone(), tk.clone()),
            EnumInnerFields::Named(tk) => (
                quote! {
                    {#tk}
                },
                quote! {
                    (#tk)
                },
            ),
            EnumInnerFields::Unit => (
                quote! {},
                quote! {
                    (#source::#rename)
                },
            ),
        };

        match extern_fn {
            Some(ext) => quote! {
                #source::#rename #match_ident => #ext:: #fn_tmpl #fn_args
            },
            None => quote! {
                #source::#rename #match_ident => #fn_tmpl #fn_args
            },
        }
    }
}
