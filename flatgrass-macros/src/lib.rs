use proc_macro::TokenStream;
use quote::quote;

mod func;

mod parse;

/// Marks a function as the entry point of your module.
///
/// This function is called when your module is first required from Lua.\
/// For this to work, it needs to be declared at the root of your library and
/// to be paired with another function marked with `#[flatgrass::exit]`.
///
/// # Examples
///
/// ```
/// #[flatgrass::entry]
/// pub fn entry() {
///   printfg!("Hello from binary module!");
/// }
/// ```
#[proc_macro_attribute]
pub fn entry(attr: TokenStream, item: TokenStream) -> TokenStream {
	let _ = syn::parse_macro_input!(attr as syn::parse::Nothing);
	let entry_fn = syn::parse_macro_input!(item as parse::EntryFn);
	func::generate_entry(entry_fn).into()
}

/// Marks a function as the exit point of your module.
///
/// For this to work, it needs to be declared at the root of your library and
/// to be paired with another function marked with `#[flatgrass::entry]`.
///
/// # Examples
///
/// ```
/// #[flatgrass::exit]
/// pub fn exit() {
///   printfg!("Goodbye from binary module!");
/// }
/// ```
#[proc_macro_attribute]
pub fn exit(attr: TokenStream, item: TokenStream) -> TokenStream {
	let _ = syn::parse_macro_input!(attr as syn::parse::Nothing);
	let exit_fn = syn::parse_macro_input!(item as parse::ExitFn);
	func::generate_exit(exit_fn).into()
}

/// Generates the necessary glue code to call a function from Lua.
///
/// # Examples
///
/// ```
/// #[flatgrass::function]
/// pub fn add(a: f32, b: f32) -> f32 {
///   a + b
/// }
/// ```
#[proc_macro_attribute]
pub fn function(attr: TokenStream, item: TokenStream) -> TokenStream {
	let _ = syn::parse_macro_input!(attr as syn::parse::Nothing);
	let lua_fn = syn::parse_macro_input!(item as parse::LuaFn);
	func::generate_func(lua_fn).into()
}

/// Returns a raw Lua function containing glue code to call the given Rust function from Lua.
///
/// This can only be used on functions previously annotated with the `#[function]` attribute.
///
/// # Examples
///
/// ```
/// #[flatgrass::entry]
/// pub fn entry() {
///   let globals = Table::globals();
///   globals.raw_set("add", cfunction!(add));
/// }
///
/// #[flatgrass::function]
/// pub fn add(a: f32, b: f32) -> f32 {
///   a + b
/// }
/// ```
#[proc_macro]
pub fn cfunction(input: TokenStream) -> TokenStream {
	let lua_fn = syn::parse_macro_input!(input as parse::LuaFnDecl);
	let (ident, turbofish) = (lua_fn.ident, lua_fn.turbofish);
	quote! { #ident::cfunction #turbofish () }.into()
}
