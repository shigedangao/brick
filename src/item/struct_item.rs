use super::ProcessItem;
use crate::attributes::BrickAttributes;
use crate::fields::BrickFieldArgs;
use proc_macro2::TokenStream;
use quote::quote;
use syn::punctuated::Punctuated;
use syn::{ItemStruct, Token};

const FIELD_NAME: &str = "brick_field";

impl ProcessItem for ItemStruct {
    fn process_item(&mut self, attrs: BrickAttributes) -> TokenStream {
        let mut processed_fields = Vec::new();

        for field in self.fields.clone() {
            let name = field.ident.clone().expect("Expect to found a name");

            let mut field_attrs = Vec::new();

            for attr in field.attrs {
                if attr.path().is_ident(FIELD_NAME) {
                    let meta: Punctuated<BrickFieldArgs, Token![,]> =
                        attr.parse_args_with(Punctuated::parse_terminated).unwrap();
                    field_attrs.extend(meta.into_iter());
                }
            }

            processed_fields.push(BrickFieldArgs::create_template(name.clone(), field_attrs));
        }

        // Use to remove the attributes brick_field from the AST so that it doesn't get printed
        self.fields.iter_mut().for_each(|field| {
            field.attrs.retain(|attr| !attr.path().is_ident(FIELD_NAME));
        });

        let expanded = attrs.create_ops_template(self.ident.clone(), processed_fields);

        quote! {
            #self
            #expanded
        }
    }
}
