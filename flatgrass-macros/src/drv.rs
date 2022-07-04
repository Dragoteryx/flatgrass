use super::*;

pub fn impl_ref(attrs: Vec<syn::Attribute>) -> bool {
  attrs.into_iter().any(|attr| if let Some(ident) = attr.path.get_ident() {
    ident == "pushtolua" && {
      true
    }
  } else {
    false
  })
}

pub fn add_trait_bounds(mut generics: syn::Generics) -> syn::Generics {
  generics.params.iter_mut()
    .for_each(|param| if let syn::GenericParam::Type(tp) = param {
      tp.bounds.push(syn::parse_quote!(::flatgrass::lua::traits::PushToLua));
    });

  generics
}