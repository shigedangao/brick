use crate::attributes::BrickAttributes;

pub mod enum_item;
pub mod struct_item;

pub(crate) trait ProcessItem {
    fn process_item(&mut self, attrs: BrickAttributes) -> proc_macro2::TokenStream;
}
