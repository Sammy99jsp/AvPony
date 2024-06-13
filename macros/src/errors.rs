use crate::{fatal_error, q_attr};

#[allow(non_snake_case)]
mod ErrorI {
    use crate::{q, spanned::Span};

    pub fn path() -> syn::Path {
        q!(crate::utils::ErrorI)
    }

    pub fn to_report(expr: syn::Expr) -> syn::Expr {
        let ErrorI = self::path();
        q!(#ErrorI::to_report(#expr))
    }

    pub fn impl_for(en @ syn::ItemEnum { ident, .. }: &syn::ItemEnum) -> syn::ItemImpl {
        let ErrorI = self::path();
        let Span = Span::path();

        let vars = en
            .variants
            .iter()
            .map(|var| var.ident.clone())
            .map(|v_ident| {
                let expr = self::to_report(q!(v));
                let arm: syn::Arm = q!(Self::#v_ident(v) => #expr,);
                arm
            });

        q!(
            impl #ErrorI for #ident {
                fn to_report(self) -> ariadne::Report<'static, #Span> {
                    match self {
                        #(#vars)*
                    }
                }
            }
        )
    }
}

pub fn generate_error_enum(tokens: proc_macro::TokenStream) -> Vec<syn::Item> {
    let Ok(mut en) = syn::parse::<syn::ItemEnum>(tokens) else {
        return fatal_error(None, "Expected an enum declaration.");
    };

    en.attrs
        .extend(q_attr!(#[derive(Debug, Clone, PartialEq, avpony_macros::Spanned)]));

    let impl_errori = ErrorI::impl_for(&en);

    vec![syn::Item::Enum(en), syn::Item::Impl(impl_errori)]
}
