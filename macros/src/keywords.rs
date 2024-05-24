pub fn get_keyword_structs(kw_mod: &syn::ItemMod) -> impl Iterator<Item = String> + '_ {
    kw_mod
        .content.as_ref()
        .map(|(_, a)| a)
        .into_iter()
        .flatten()
        .filter_map(|it| match it {
            syn::Item::Struct(st) => Some(st),
            _ => None,
        })
        .map(|syn::ItemStruct {ident, ..}| ident.to_string().to_lowercase())
}
