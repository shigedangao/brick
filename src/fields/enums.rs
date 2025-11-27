use super::*;

impl BrickFieldArgs {
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
                Some(f) => enum_builder::generate_enum_fn(
                    source,
                    name,
                    rename,
                    fn_from_extern,
                    f,
                    &enum_fields,
                ),
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
        original_field_name: Ident,
        rename: Option<Ident>,
        extern_fn: Option<Ident>,
        fn_tmpl: Ident,
        enum_inner_fields: &EnumInnerFields,
    ) -> TokenStream {
        let fn_call_template = match extern_fn {
            Some(ext) => quote! { #ext:: #fn_tmpl },
            None => quote! { #fn_tmpl },
        };

        let (source_idents, complete_fn_call) = match enum_inner_fields {
            EnumInnerFields::Unnamed(tk) => (
                tk.clone(),
                quote! {
                Self::#original_field_name(#fn_call_template(#tk)) },
            ),
            EnumInnerFields::Named(tk) => (
                quote! {
                    {#tk}
                },
                quote! {
                    #fn_call_template (#tk)
                },
            ),
            EnumInnerFields::Unit => (
                quote! {},
                quote! {
                    #fn_call_template (#source::#rename)
                },
            ),
        };

        quote! {
            #source::#rename #source_idents => #complete_fn_call
        }
    }
}
