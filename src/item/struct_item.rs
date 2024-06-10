use super::ProcessItem;
use crate::attributes::BrickAttributes;
use crate::fields::BrickFieldArgs;
use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{Ident, ItemStruct};

impl ProcessItem for ItemStruct {
    fn process_item(&mut self, attrs: BrickAttributes) -> TokenStream {
        let fields = self
            .clone()
            .fields
            .iter()
            .map(|item| {
                let field_name = &item.ident;
                let fields_args: Option<BrickFieldArgs> = item.attrs.iter().find_map(|attr| {
                    attr.path()
                        .is_ident("brick_field")
                        .then(|| attr.parse_args().expect("Expect to parse brick_fields arg"))
                });

                match fields_args {
                    Some(BrickFieldArgs::ConvertFieldFn(convert_field)) => {
                        let func = Ident::new(&convert_field.value(), Span::call_site());

                        proc_macro2::TokenStream::from(quote! {
                            #field_name: #func(arg.#field_name)
                        })
                    }
                    _ => proc_macro2::TokenStream::from(quote! { #field_name: arg.#field_name }),
                }
            })
            .collect::<Vec<_>>();

        // Use to remove the attributes brick_field from the AST so that it doesn't get printed
        self.fields.iter_mut().for_each(|field| {
            field
                .attrs
                .retain(|attr| !attr.path().is_ident("brick_field"));
        });

        let expanded = attrs.create_ops_template(self.ident.clone(), fields);

        quote! {
            #self
            #expanded
        }
    }
}
