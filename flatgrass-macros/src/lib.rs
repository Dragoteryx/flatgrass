use proc_macro2::TokenStream as TokenStream2;
use proc_macro::TokenStream;
use syn::spanned::Spanned;

#[macro_use]
mod util; use util::*;

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