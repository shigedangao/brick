use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{meta::ParseNestedMeta, spanned::Spanned, Ident, LitStr, Result, Type};

#[derive(Default, PartialEq)]
pub enum ConverterType {
    #[default]
    From,
    TryFrom,
}

#[derive(Default)]
pub struct BrickAttributes {
    pub converter: ConverterType,
    pub source_struct: Option<Ident>,
    pub source_enum: Option<Ident>,
    pub error_kind: Option<LitStr>,
}

impl BrickAttributes {
    pub fn parse(&mut self, meta: ParseNestedMeta) -> Result<()> {
        if meta.path.get_ident().is_none() {
            return Err(syn::Error::new(meta.path.span(), "Unknown attribute"));
        }

        let ident = meta.path.get_ident().unwrap();
        match ident.to_string().as_str() {
            "converter" => {
                let converter: LitStr = meta.value()?.parse()?;
                self.converter = match converter.value().as_str() {
                    "From" => ConverterType::From,
                    "TryFrom" => ConverterType::TryFrom,
                    _ => ConverterType::From,
                };

                Ok(())
            }
            "source_struct" => {
                let source_struct: Option<LitStr> = meta.value()?.parse()?;
                if let Some(src) = source_struct {
                    self.source_struct = Some(Ident::new(&src.value(), Span::call_site()));
                }

                Ok(())
            }
            "source_enum" => {
                let source_enum: Option<LitStr> = meta.value()?.parse()?;
                if let Some(src) = source_enum {
                    self.source_enum = Some(Ident::new(&src.value(), Span::call_site()));
                }

                Ok(())
            }
            "try_error_kind" => {
                self.error_kind = Some(meta.value()?.parse()?);

                Ok(())
            }
            _ => Err(syn::Error::new(ident.span(), "Unknown attribute")),
        }
    }

    /// Create the conversion template for the target item (struct or enum)
    ///
    /// # Arguments
    ///
    /// * `target_ident` - The target struct identifier
    /// * `transform_fields` - The transformed fields
    pub fn create_ops_template(
        &self,
        target_ident: Ident,
        transform_fields: Vec<TokenStream>,
    ) -> TokenStream {
        let (source, fields) = if let Some(source_struct) = &self.source_struct {
            (
                source_struct,
                quote! {
                    Self {
                        #(#transform_fields),*
                    }
                },
            )
        } else if let Some(source_enum) = &self.source_enum {
            (
                source_enum,
                quote! {
                    #(#transform_fields),*
                },
            )
        } else {
            panic!("Expect source_struct or source_enum to be provided");
        };

        match self.converter {
            ConverterType::From => {
                quote! {
                    impl From<#source> for #target_ident {
                        fn from(arg: #source) -> Self {
                            #fields
                        }
                    }
                }
            }
            ConverterType::TryFrom => {
                let error_kind = self
                    .error_kind
                    .as_ref()
                    .expect("Expect try_error_kind to be provided");
                let error_kind_ident: Type =
                    syn::parse_str(&error_kind.value()).expect("Expect to parse error_kind");

                quote! {
                    impl TryFrom<#source> for #target_ident {
                        type Error = #error_kind_ident;

                        fn try_from(arg: #source) -> Result<Self, Self::Error> {
                            Ok(#fields)
                        }
                    }
                }
            }
        }
    }
}
