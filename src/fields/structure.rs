use super::*;

impl BrickFieldArgs {
    /// Create the struct template which will be used inside the field to map the path src: target
    ///
    /// # Arguments
    /// * `name` - The name of the struct template.
    /// * `fields` - The fields of the struct template.
    pub(crate) fn create_struct_template(name: Ident, fields: Vec<Self>) -> TokenStream {
        let mut from_field_name: Option<Ident> = Some(name.clone());
        let mut fn_from_extern: Option<syn::Ident> = None;
        let mut func: Option<Ident> = None;
        let mut to_skip = false;
        let mut is_fallible = false;

        for field in fields {
            if let Self::Rename(n) = field.to_owned() {
                from_field_name = Some(Ident::new(&n.value(), Span::call_site()));
            }

            if let Self::FnFromExtern(t) = field.to_owned() {
                fn_from_extern = Some(Ident::new(&t.value(), Span::call_site()));
            }

            if let Self::ConvertFieldFn(f) = field.to_owned() {
                func = Some(Ident::new(&f.value(), Span::call_site()));
            }

            if let Self::IsFallible(r) = field.to_owned() {
                is_fallible = r.value();
            }

            // In the case where we exclude the field, we just skip to output that field.
            if let Self::Exclude(e) = field.to_owned()
                && e.value()
            {
                to_skip = true;
            }
        }

        let res_call = match is_fallible {
            true => quote! {
                (arg.#from_field_name)?
            },
            false => quote! {
                (arg.#from_field_name)
            },
        };

        match to_skip {
            true => quote! {
                #name: Default::default()
            },
            false => match func {
                Some(f) => match fn_from_extern {
                    Some(t) => quote! {
                        #name: #t::#f #res_call
                    },
                    None => quote! {
                        #name: #f #res_call
                    },
                },
                None => quote! { #name: arg.#from_field_name },
            },
        }
    }
}
