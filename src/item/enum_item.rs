use super::ProcessItem;
use crate::fields::BrickFieldArgs;
use crate::item::FIELD_NAME;
use crate::{attributes::BrickAttributes, item::SupportedType};
use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;
use syn::spanned::Spanned;
use syn::{Fields, ItemEnum, Token, punctuated::Punctuated};

impl ProcessItem for ItemEnum {
    fn process(
        &mut self,
        attrs: BrickAttributes,
        supported_type: SupportedType,
    ) -> proc_macro2::TokenStream {
        let target = self.ident.clone();

        let mut field_tk = Vec::new();
        for item in self.variants.clone() {
            let field_name = item.ident;

            let enum_inner_fields = process_enum_inner_fields(item.fields);

            let mut field_attrs = Vec::new();
            for attr in item.attrs {
                if attr.path().is_ident(super::FIELD_NAME) {
                    let meta: Punctuated<BrickFieldArgs, Token![,]> =
                        attr.parse_args_with(Punctuated::parse_terminated).unwrap();

                    field_attrs.extend(meta.into_iter());
                }
            }

            field_tk.push(BrickFieldArgs::create_enum_template(
                field_name,
                attrs.source.clone(),
                field_attrs,
                enum_inner_fields,
            ));
        }

        let expanded =
            attrs.generate_conversion_template(target.clone(), field_tk, supported_type.clone());

        // Remove the #[brick(field)] attribute from the variants before passing to the TokenStream
        self.variants.iter_mut().for_each(|field| {
            field.attrs.retain(|attr| !attr.path().is_ident(FIELD_NAME));
        });

        quote! {
            #self
            #expanded
        }
    }
}

/// Process the enum fields e.g Enum::Variant(arg1, arg2)
///
/// # Description
/// This function will take the enum fields and process them into a TokenStream
/// - Unnamed fields will generate a tuple of arguments in the following format (arg_0, arg_1, ...)
/// - Unit will just produce an empty TokenStream
/// - Named fields will generate a tuple of arguments in the following format (arg_0, arg_1, ...)
///
/// /!\ Named fields are not supported yet
fn process_enum_inner_fields(fields: Fields) -> TokenStream {
    let enum_arg_fields: Vec<TokenStream> = match fields {
        Fields::Unnamed(un) => un
            .unnamed
            .into_iter()
            .enumerate()
            .map(|(idx, field)| {
                let ident = Ident::new(&format!("arg_{}", idx), field.span());

                quote! { #ident }
            })
            .collect(),
        _ => vec![],
    };

    match enum_arg_fields.is_empty() {
        true => quote! {},
        false => quote! {
            (#(#enum_arg_fields),*)
        },
    }
}
