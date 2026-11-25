use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

#[proc_macro_derive(HomogeneousFields)]
pub fn derive_homogeneous_fields(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    match expand_derive(&input) {
        Ok(expanded) => expanded.into(),
        Err(error) => error.to_compile_error().into(),
    }
}

fn expand_derive(input: &DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    let fields = match &input.data {
        Data::Struct(data) => &data.fields,
        _ => {
            return Err(syn::Error::new_spanned(
                input,
                "HomogeneousFields can only be derived for structs",
            ));
        }
    };

    let field_data: Vec<_> = match fields {
        Fields::Named(fields) => fields
            .named
            .iter()
            .map(|field| {
                let member = field.ident.as_ref().expect("named field has an identifier");
                (
                    syn::Member::Named(member.clone()),
                    member.to_string(),
                    &field.ty,
                )
            })
            .collect(),
        Fields::Unnamed(fields) => fields
            .unnamed
            .iter()
            .enumerate()
            .map(|(index, field)| {
                (
                    syn::Member::Unnamed(index.into()),
                    index.to_string(),
                    &field.ty,
                )
            })
            .collect(),
        Fields::Unit => Vec::new(),
    };

    let Some((_, _, item_type)) = field_data.first() else {
        return Err(syn::Error::new_spanned(
            input,
            "HomogeneousFields requires a struct with at least one field",
        ));
    };

    let name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let owned_fields = field_data
        .iter()
        .map(|(member, field_name, _)| quote! { (#field_name, self.#member), })
        .collect::<Vec<_>>();
    let shared_fields = field_data
        .iter()
        .map(|(member, field_name, _)| quote! { (#field_name, &self.#member), })
        .collect::<Vec<_>>();
    let mutable_fields = field_data
        .iter()
        .map(|(member, field_name, _)| quote! { (#field_name, &mut self.#member), })
        .collect::<Vec<_>>();
    let field_count = field_data.len();

    Ok(quote! {
        impl #impl_generics ::homogeneous_fields::HomogeneousFields
            for #name #ty_generics #where_clause
        {
            type Item = #item_type;
            type IntoIter = ::std::array::IntoIter<(&'static str, Self::Item), #field_count>;
            type Iter<'__homogeneous_fields> =
                ::std::array::IntoIter<
                    (&'static str, &'__homogeneous_fields Self::Item),
                    #field_count
                >
            where
                Self: '__homogeneous_fields;
            type IterMut<'__homogeneous_fields> =
                ::std::array::IntoIter<
                    (&'static str, &'__homogeneous_fields mut Self::Item),
                    #field_count
                >
            where
                Self: '__homogeneous_fields;

            fn into_fields(self) -> Self::IntoIter {
                ::std::iter::IntoIterator::into_iter([
                    #(#owned_fields)*
                ])
            }

            fn fields(&self) -> Self::Iter<'_> {
                ::std::iter::IntoIterator::into_iter([
                    #(#shared_fields)*
                ])
            }

            fn fields_mut(&mut self) -> Self::IterMut<'_> {
                ::std::iter::IntoIterator::into_iter([
                    #(#mutable_fields)*
                ])
            }
        }
    })
}
