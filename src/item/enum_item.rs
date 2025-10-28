use super::ProcessItem;
use crate::fields::BrickFieldArgs;
use crate::item::FIELD_NAME;
use crate::{attributes::BrickAttributes, item::SupportedType};
use quote::quote;
use syn::{ItemEnum, Token, punctuated::Punctuated};

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
