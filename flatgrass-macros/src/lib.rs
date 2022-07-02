use quote::{quote, quote_spanned, format_ident};
use proc_macro2::TokenStream as TokenStream2;
use proc_macro::TokenStream;
use syn::spanned::Spanned;

#[macro_use]
mod util; use util::*;

/// Used to mark the entry function of your module.
/// This macro changes the signature of the function to one compatible with Lua's C API.
/// ```
/// #[flatgrass::open]
/// pub fn gmod13_open(lua: Lua) {
///   printfg!(lua, "Hello!");
/// }
/// ```
#[proc_macro_attribute]
pub fn open(_: TokenStream, item: TokenStream) -> TokenStream {
  let item = syn::parse_macro_input!(item as syn::ItemFn);

  check_valid(&item)
    .or_else(|| check_name!(item, "gmod13_open"))
    .unwrap_or_else(|| wrap_function(item))
    .into()
}

/// Used to mark the exit function of your module.
/// This macro changes the signature of the function to one compatible with Lua's C API.
/// ```
/// #[flatgrass::close]
/// pub fn gmod13_close(lua: Lua) {
///   printfg!(lua, "Goodbye!");
/// }
/// ```
#[proc_macro_attribute]
pub fn close(_: TokenStream, item: TokenStream) -> TokenStream {
  let item = syn::parse_macro_input!(item as syn::ItemFn);

  check_valid(&item)
    .or_else(|| check_name!(item, "gmod13_close"))
    .unwrap_or_else(|| wrap_function(item))
    .into()
}

/// This macro changes the signature of the function to one compatible with Lua's C API.
/// ```
/// #[flatgrass:function]
/// pub is_even(n: isize) -> bool {
///   n % 2 == 0
/// }
/// ```
#[proc_macro_attribute]
pub fn function(_: TokenStream, item: TokenStream) -> TokenStream {
  let item = syn::parse_macro_input!(item as syn::ItemFn);

  check_valid(&item)
    .unwrap_or_else(|| wrap_function(item))
    .into()
}