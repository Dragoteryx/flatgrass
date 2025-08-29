use proc_macro::TokenStream;

mod func;

mod derive;

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
pub fn entry(args: TokenStream, input: TokenStream) -> TokenStream {
	let _ = syn::parse_macro_input!(args as syn::parse::Nothing);
	let func = syn::parse_macro_input!(input as syn::ItemFn);
	func::generate_entry(&func).into()
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
pub fn exit(args: TokenStream, input: TokenStream) -> TokenStream {
	let _ = syn::parse_macro_input!(args as syn::parse::Nothing);
	let func = syn::parse_macro_input!(input as syn::ItemFn);
	func::generate_exit(&func).into()
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
pub fn function(args: TokenStream, input: TokenStream) -> TokenStream {
	let _ = syn::parse_macro_input!(args as syn::parse::Nothing);
	let func = syn::parse_macro_input!(input as syn::ItemFn);
	func::generate_func(&func).into()
}

/// Automatically implements the `ToLua` trait for structs
/// by converting each field to a Lua table entry.
/// 
/// # Examples
/// 
/// ```
/// #[derive(ToLua)]
/// struct Player {
/// 	name: String,
/// 	score: u32,
/// }
/// ```
#[proc_macro_derive(ToLua)]
pub fn to_lua(input: TokenStream) -> TokenStream {
	let input = syn::parse_macro_input!(input as syn::DeriveInput);
	derive::to_lua(input).into()
}
