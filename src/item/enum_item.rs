use super::ProcessItem;
use crate::{attributes::BrickAttributes, item::SupportedType};
use quote::quote;
use syn::ItemEnum;

impl ProcessItem for ItemEnum {
    fn process(
        &mut self,
        attrs: BrickAttributes,
        supported_type: SupportedType,
    ) -> proc_macro2::TokenStream {
        let target = self.ident.clone();
        let variants = self.variants.clone();

        let variants = variants
            .into_iter()
            .map(|var| {
                let field_name = var.ident;
                attrs.generate_conversion_template(
                    target.clone(),
                    vec![quote! { Self::#field_name }],
                    supported_type.clone(),
                )
            })
            .collect::<Vec<_>>();

        quote! {
            #self
            #(#variants)*
        }
    }
}
