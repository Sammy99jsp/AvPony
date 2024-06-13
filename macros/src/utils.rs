macro_rules! q {
    ($($t: tt)*) => {{
        let tokens = quote::quote! { $($t)* };
        syn::parse2(tokens.clone())
            .expect(&format!("Error on {}:{}\n\nTokens: {tokens}", file!(), line!()))
    }}
}

macro_rules! q_attr {
    ($($t : tt)*) => {{
        struct _S(Vec<syn::Attribute>);

        impl syn::parse::Parse for _S {
            fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
                let parsed = syn::Attribute::parse_outer(input).expect("Could not parse outer attribute!");
                Ok(_S(parsed))
            }
        }

        syn::parse2::<_S>(
            quote::quote! { $($t)* },
        )
        .unwrap()
        .0
    }};
}

pub(crate) use q;
pub(crate) use q_attr;

pub(crate) trait ToType: Sized + quote::ToTokens {
    fn to_type(self) -> syn::Type;
}

impl ToType for syn::Path {
    fn to_type(self) -> syn::Type {
        syn::Type::Path(syn::TypePath {
            qself: None,
            path: self,
        })
    }
}

pub(crate) fn ident(s: &str) -> syn::Ident {
    syn::Ident::new(s, proc_macro2::Span::call_site())
}

pub(crate) fn fatal_error<T: Default>(
    span: impl Into<Option<proc_macro2::Span>>,
    msg: impl Into<String>,
) -> T {
    let span = span.into().unwrap_or(proc_macro2::Span::call_site());

    proc_macro::Diagnostic::spanned(span.unwrap(), proc_macro::Level::Error, msg).emit();

    T::default()
}
