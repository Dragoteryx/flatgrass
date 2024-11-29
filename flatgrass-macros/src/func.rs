use proc_macro2::*;
use quote::{quote, quote_spanned};
use syn::spanned::Spanned;
use syn::*;

pub fn generate_entry(func: &ItemFn) -> TokenStream {
	let tokens = generate_func(func);
	let ident = &func.sig.ident;

	let mut errors = Vec::new();
	for param in &func.sig.generics.params {
		if let GenericParam::Type(param) = param {
			errors.push(
				quote_spanned!(param.span() => compile_error!("the entry function cannot have type parameters")),
			);
		} else if let GenericParam::Const(param) = param {
			errors.push(
				quote_spanned!(param.span() => compile_error!("the entry function cannot have const parameters")),
			);
		}
	}

	if !errors.is_empty() {
		quote! {
			#tokens

			#(#errors;)*

			#[no_mangle]
			#[doc(hidden)]
			pub unsafe extern "C-unwind" fn gmod13_open(state: *mut ::flatgrass::ffi::lua_State) -> ::flatgrass::ffi::c_int {
				use crate::{gmod13_open, gmod13_close};
				0
			}
		}
	} else {
		quote! {
			#tokens

			#[no_mangle]
			#[doc(hidden)]
			pub unsafe extern "C-unwind" fn gmod13_open(state: *mut ::flatgrass::ffi::lua_State) -> ::flatgrass::ffi::c_int {
				use crate::{gmod13_open, gmod13_close};
				::flatgrass::lua::Lua::init(state, |lua| {
					lua.entry();
					#ident::to_lua(state)
				})
			}
		}
	}
}

pub fn generate_exit(func: &ItemFn) -> TokenStream {
	let tokens = generate_func(func);
	let ident = &func.sig.ident;

	let mut errors = Vec::new();
	for param in &func.sig.generics.params {
		if let GenericParam::Type(param) = param {
			errors.push(
				quote_spanned!(param.span() => compile_error!("the exit function cannot have type parameters")),
			);
		} else if let GenericParam::Const(param) = param {
			errors.push(
				quote_spanned!(param.span() => compile_error!("the exit function cannot have const parameters")),
			);
		}
	}

	if !errors.is_empty() {
		quote! {
			#tokens

			#(#errors;)*

			#[no_mangle]
			#[doc(hidden)]
			pub unsafe extern "C-unwind" fn gmod13_close(state: *mut ::flatgrass::ffi::lua_State) -> ::flatgrass::ffi::c_int {
				use crate::{gmod13_open, gmod13_close};
				0
			}
		}
	} else {
		quote! {
			#tokens

			#[no_mangle]
			#[doc(hidden)]
			pub unsafe extern "C-unwind" fn gmod13_close(state: *mut ::flatgrass::ffi::lua_State) -> ::flatgrass::ffi::c_int {
				use crate::{gmod13_open, gmod13_close};
				::flatgrass::lua::Lua::init(state, |lua| {
					let res = #ident::to_lua(state);
					lua.exit();
					res
				})
			}
		}
	}
}

pub fn generate_func(func: &ItemFn) -> TokenStream {
	let (impl_generics, type_generics, where_clause) = func.sig.generics.split_for_impl();
	let generics_turbofish = type_generics.as_turbofish();
	let ident = &func.sig.ident;
	let vis = &func.vis;

	let mut errors = Vec::new();
	let ret_span = match &func.sig.output {
		ReturnType::Default => func.sig.output.span(),
		ReturnType::Type(_, ty) => ty.span(),
	};

	if let Some(unsafety) = &func.sig.unsafety {
		errors.push(
			quote_spanned!(unsafety.span => compile_error!("Lua functions cannot be unsafe")),
		);
	}

	if let Some(asyncness) = &func.sig.asyncness {
		errors.push(
			quote_spanned!(asyncness.span => compile_error!("Lua functions cannot be async (yet)")),
		);
	}

	if !errors.is_empty() {
		quote! {
			#func

			#(#errors;)*

			#[doc(hidden)]
			#vis mod #ident {
				use super::*;

				pub unsafe extern "C-unwind" fn to_lua #impl_generics (state: *mut ::flatgrass::ffi::lua_State) -> ::flatgrass::ffi::c_int #where_clause {
					0
				}
			}
		}
	} else {
		let args = func.sig.inputs.iter().map(|input| {
			quote_spanned! { input.span() =>
				match ::flatgrass::lua::traits::function::LuaFnParam::lua_fn_param(lua, &mut arg, &mut upv) {
					::std::result::Result::Ok(value) => value,
					::std::result::Result::Err(err) => {
						lua.stack().clear();
						lua.stack().push_any(err);
						return ::std::option::Option::None;
					}
				}
			}
		});

		let body = if func.sig.asyncness.is_some() {
			quote_spanned! { ret_span =>


			}
		} else {
			quote_spanned! { ret_span =>
				match ::flatgrass::lua::traits::function::LuaFnReturn::lua_fn_return(#ident #generics_turbofish (#(#args),*), lua) {
					::std::result::Result::Ok(::flatgrass::lua::traits::function::Return::Values(values)) =>
						::std::option::Option::Some(::flatgrass::lua::traits::function::Return::Values(lua.stack().push_many(values))),
					::std::result::Result::Ok(::flatgrass::lua::traits::function::Return::Yield(values)) =>
						::std::option::Option::Some(::flatgrass::lua::traits::function::Return::Yield(lua.stack().push_many(values))),
					::std::result::Result::Err(err) => {
						lua.stack().clear();
						lua.stack().push_any(err);
						::std::option::Option::None
					}
				}
			}
		};

		quote! {
			#func

			#[doc(hidden)]
			#vis mod #ident {
				use super::*;

				pub unsafe extern "C-unwind" fn to_lua #impl_generics (state: *mut ::flatgrass::ffi::lua_State) -> ::flatgrass::ffi::c_int #where_clause {
					match ::flatgrass::lua::Lua::init(state, |lua| {
						let (mut arg, mut upv) = (1, 1);
						#body
					}) {
						::std::option::Option::None => ::flatgrass::ffi::lua_error(state),
						::std::option::Option::Some(ret) => match ret {
							::flatgrass::lua::traits::function::Return::Yield(i) => ::flatgrass::ffi::lua_yield(state, i),
							::flatgrass::lua::traits::function::Return::Values(i) => i,
						}
					}
				}
			}
		}
	}
}
