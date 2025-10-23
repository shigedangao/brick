use crate::attributes::BrickAttributes;

pub mod enum_item;
pub mod struct_item;

#[derive(Clone)]
pub enum SupportedType {
    Struct,
    Enum,
}

pub(crate) trait ProcessItem {
    fn process(
        &mut self,
        attrs: BrickAttributes,
        supported_type: SupportedType,
    ) -> proc_macro2::TokenStream;
}
