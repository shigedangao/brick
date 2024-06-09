use super::ProcessItem;
use crate::attributes::BrickAttributes;
use quote::quote;
use syn::ItemEnum;

impl ProcessItem for ItemEnum {
    fn process_item(&mut self, attrs: BrickAttributes) -> proc_macro2::TokenStream {
        let target = self.ident.clone();
        let variants = self.variants.clone();

        let variants = variants
            .into_iter()
            .map(|var| {
                let field_name = var.ident;
                attrs.create_ops_template(target.clone(), vec![quote! { Self::#field_name }])
            })
            .collect::<Vec<_>>();

        proc_macro2::TokenStream::from(quote! {
            #self
            #(#variants)*
        })
    }
}
