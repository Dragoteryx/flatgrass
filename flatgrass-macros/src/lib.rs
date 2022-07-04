use proc_macro2::TokenStream as TokenStream2;
use proc_macro::TokenStream;
use syn::spanned::Spanned;

#[macro_use]
mod util;

#[macro_use]
mod attr; use attr::*;

#[macro_use]
mod drv; use drv::*;

/// Creates a new function that acts as the entry function of your module.
/// ```
/// #[flatgrass::entry]
/// pub fn entry(lua: Lua) {
///   printfg!(lua, "Hello!");
/// }
/// ```
#[proc_macro_attribute]
pub fn entry(_: TokenStream, item: TokenStream) -> TokenStream {
  let item = syn::parse_macro_input!(item as syn::ItemFn);
  let ident = quote::format_ident!("gmod13_open");

  check_valid(&item)
    .unwrap_or_else(|| gen_function(ident, item))
    .into()
}

/// Creates a new function that acts as the exit function of your module.
/// ```
/// #[flatgrass::exit]
/// pub fn exit(lua: Lua) {
///   printfg!(lua, "Goodbye!");
/// }
/// ```
#[proc_macro_attribute]
pub fn exit(_: TokenStream, item: TokenStream) -> TokenStream {
  let item = syn::parse_macro_input!(item as syn::ItemFn);
  let ident = quote::format_ident!("gmod13_close");

  check_valid(&item)
    .unwrap_or_else(|| gen_function(ident, item))
    .into()
}

/// Creates a new function that can be pushed to the Lua environment.
/// ```
/// #[flatgrass:function(lua_is_even)]
/// pub const is_even(n: isize) -> bool {
///   n % 2 == 0
/// }
/// ```
#[proc_macro_attribute]
pub fn function(attr: TokenStream, item: TokenStream) -> TokenStream {
  let ident = syn::parse_macro_input!(attr as syn::Ident);
  let item = syn::parse_macro_input!(item as syn::ItemFn);

  check_valid(&item)
    .unwrap_or_else(|| gen_function(ident, item))
    .into()
}

#[proc_macro_derive(PushToLua, attributes(pushtolua))]
pub fn derive_push_to_lua(item: TokenStream) -> TokenStream {
  let item = syn::parse_macro_input!(item as syn::DeriveInput);
  let generics = add_trait_bounds(item.generics);
  let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
  let ident = item.ident;

  let output = if impl_ref(item.attrs) {
    quote::quote!(
      #[automatically_derived]
      #[allow(unused_qualifications)]
      impl #impl_generics ::flatgrass::lua::traits::PushToLua for &#ident #ty_generics #where_clause {
        unsafe fn push(state: ::flatgrass::ffi::LuaState, value: Self) {
          todo!()
        }
      }

      #[automatically_derived]
      #[allow(unused_qualifications)]
      impl #impl_generics ::flatgrass::lua::traits::PushToLua for #ident #ty_generics #where_clause {
        unsafe fn push(state: ::flatgrass::ffi::LuaState, value: Self) {
          state.fg_pushvalue(&value);
        }
      }
    )
  } else {
    quote::quote!(
      #[automatically_derived]
      #[allow(unused_qualifications)]
      impl #impl_generics ::flatgrass::lua::traits::PushToLua for #ident #ty_generics #where_clause {
        unsafe fn push(state: ::flatgrass::ffi::LuaState, value: Self) {
          todo!()
        }
      }
    )
  };

  output.into()
}