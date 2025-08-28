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
			errors.push(quote_spanned! { param.span() =>
				compile_error!("the entry function cannot have type parameters")
			});
		} else if let GenericParam::Const(param) = param {
			errors.push(quote_spanned! { param.span() =>
				compile_error!("the entry function cannot have const parameters")
			});
		}
	}

	let body = match errors.is_empty() {
		false => quote! { 0 },
		true => quote! {
			if ::flatgrass::lua::Lua::init(state, |lua| {
				lua.__fg_entry();
				let func = ::flatgrass::lua::func!(#ident);
				match func.call(()) {
					Ok(_) => false,
					Err(err) => {
						lua.stack().clear();
						lua.stack().push_any(err);
						true
					},
				}
			}) {
				::flatgrass::ffi::lua_error(state)
			} else {
				0
			}
		}
	};

	quote! {
		#tokens

		#(#errors;)*

		#[doc(hidden)]
		#[unsafe(no_mangle)]
		pub unsafe extern "C-unwind" fn gmod13_open(state: *mut ::flatgrass::ffi::lua_State) -> ::flatgrass::ffi::libc::c_int {
			use crate::{gmod13_open, gmod13_close};
			#body
		}
	}
}

pub fn generate_exit(func: &ItemFn) -> TokenStream {
	let tokens = generate_func(func);
	let ident = &func.sig.ident;

	let mut errors = Vec::new();
	for param in &func.sig.generics.params {
		if let GenericParam::Type(param) = param {
			errors.push(quote_spanned! { param.span() =>
				compile_error!("the exit function cannot have type parameters")
			});
		} else if let GenericParam::Const(param) = param {
			errors.push(quote_spanned! { param.span() =>
				compile_error!("the exit function cannot have const parameters")
			});
		}
	}

	let body = match errors.is_empty() {
		false => quote! { 0 },
		true => quote! {
			if ::flatgrass::lua::Lua::init(state, |lua| {
				let func = ::flatgrass::lua::func!(#ident);
				let res = func.call(());
				lua.__fg_exit();
				match res {
					Ok(_) => false,
					Err(err) => {
						lua.stack().clear();
						lua.stack().push_any(err);
						true
					},
				}
			}) {
				::flatgrass::ffi::lua_error(state)
			} else {
				0
			}
		}
	};

	quote! {
		#tokens

		#(#errors;)*

		#[doc(hidden)]
		#[unsafe(no_mangle)]
		pub unsafe extern "C-unwind" fn gmod13_close(state: *mut ::flatgrass::ffi::lua_State) -> ::flatgrass::ffi::libc::c_int {
			use crate::{gmod13_open, gmod13_close};
			#body
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
		errors.push(quote_spanned! { unsafety.span =>
			compile_error!("Lua functions cannot be unsafe")
		});
	}

	if let Some(asyncness) = &func.sig.asyncness {
		errors.push(quote_spanned! { asyncness.span =>
			compile_error!("Lua functions cannot be async (yet)")
		});
	}

	let body = match errors.is_empty() {
		false => quote! { ::flatgrass::ffi::raw_function!(|_| 0) },
		true => {
			let args = func.sig.inputs.iter().map(|input| {
				quote_spanned! { input.span() =>
					match ::flatgrass::lua::traits::LuaFnParam::lua_fn_param(lua, &mut arg, &mut upv) {
						::core::result::Result::Ok(value) => value,
						::core::result::Result::Err(err) => {
							lua.stack().clear();
							lua.stack().push_any(err);
							return ::core::option::Option::None;
						}
					}
				}
			});

			let call = match &func.sig.asyncness {
				Some(_) => quote_spanned! { ret_span => },
				None => quote_spanned! { ret_span =>
					match ::flatgrass::lua::traits::LuaFnReturn::lua_fn_return(#ident #generics_turbofish (#(#args),*), lua) {
						::core::result::Result::Ok(::flatgrass::lua::traits::Return::Values(values)) =>
							::core::option::Option::Some(::flatgrass::lua::traits::Return::Values(lua.stack().push_many(values))),
						::core::result::Result::Ok(::flatgrass::lua::traits::Return::Yield(values)) =>
							::core::option::Option::Some(::flatgrass::lua::traits::Return::Yield(lua.stack().push_many(values))),
						::core::result::Result::Err(err) => {
							lua.stack().clear();
							lua.stack().push_any(err);
							::core::option::Option::None
						}
					}
				}
			};

			quote! {
				pub unsafe extern "C-unwind" fn func #impl_generics (state: *mut ::flatgrass::ffi::lua_State) -> ::flatgrass::ffi::libc::c_int #where_clause {
					match ::flatgrass::lua::Lua::init(state, |lua| {
						let (mut arg, mut upv) = (1, 1);
						#call
					}) {
						::core::option::Option::None => ::flatgrass::ffi::lua_error(state),
						::core::option::Option::Some(ret) => match ret {
							::flatgrass::lua::traits::Return::Yield(i) => ::flatgrass::ffi::lua_yield(state, i),
							::flatgrass::lua::traits::Return::Values(i) => i,
						}
					}
				}

				func::<#type_generics>
			}
		}
	};

	quote! {
		#func

		#(#errors;)*

		#[doc(hidden)]
		#vis mod #ident {
			use super::*;

			pub const fn to_lua #impl_generics () -> ::flatgrass::ffi::lua_CFunction #where_clause {
				#body
			}
		}
	}
}
