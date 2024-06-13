use crate::utils::q_attr;

#[allow(non_snake_case)]
mod From {
    use crate::{
        spanned::genericize_path,
        utils::{q, ToType},
    };

    mod from {
        use crate::utils::q;

        pub fn impl_for(variant: &syn::Ident, value_ty: &syn::Type) -> syn::ImplItemFn {
            q!(fn from(value: #value_ty) -> Self {
                Self::#variant(value)
            })
        }
    }

    pub fn impl_for(
        parent: syn::Path,
        ident: &syn::Ident,
        generics: &syn::Generics,
    ) -> syn::ItemImpl {
        let ty = genericize_path(&ident.clone().into(), generics).to_type();
        let func = syn::ImplItem::Fn(from::impl_for(ident, &ty));

        q!(
            impl #generics From<#ty> for #parent {
                #func
            }
        )
    }
}

pub fn impl_struct(parent: syn::Path, mut st: syn::ItemStruct) -> Vec<syn::Item> {
    st.attrs
        .extend(q_attr!(#[derive(Debug, Clone, PartialEq, avpony_macros::Spanned)]));

    let impl_ = syn::Item::Impl(From::impl_for(parent, &st.ident, &st.generics));
    vec![syn::Item::Struct(st), impl_]
}
