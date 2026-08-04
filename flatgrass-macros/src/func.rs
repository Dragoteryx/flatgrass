use proc_macro2::*;
use quote::{format_ident, quote};
use syn::spanned::Spanned;
use syn::*;

pub fn generate_entry(func: &ItemFn) -> TokenStream {
	let ident = format_ident!("{}", func.sig.ident.to_string());
	let tokens = generate_func(func);
	let mut errors = Vec::new();

	for param in &func.sig.generics.params {
		if let GenericParam::Type(param) = param {
			let err = Error::new(
				param.span(),
				"the entry function cannot have type parameters",
			);
			errors.push(err.to_compile_error());
		} else if let GenericParam::Const(param) = param {
			let err = Error::new(
				param.span(),
				"the entry function cannot have const parameters",
			);
			errors.push(err.to_compile_error());
		}
	}

	let body = match errors.is_empty() {
		false => quote! { 0 },
		true => quote! {
			if ::flatgrass::lua::Lua::enter(__fg_state, |__fg_lua| {
				let __fg_func = ::flatgrass::lua::Function::new(::flatgrass::cfunction!(#ident));
				__fg_lua.__fg_entry();
				match ::flatgrass::lua::call!(__fg_func) {
					Ok(_) => false,
					Err(__fg_err) => {
						__fg_lua.stack().clear();
						__fg_lua.stack().push_any(__fg_err);
						true
					},
				}
			}) {
				::flatgrass::ffi::lua_error(__fg_state)
			} else {
				0
			}
		},
	};

	quote! {
		#tokens

		#(#errors)*

		#[doc(hidden)]
		#[unsafe(no_mangle)]
		pub unsafe extern "C-unwind" fn gmod13_open(__fg_state: *mut ::flatgrass::ffi::lua_State) -> ::flatgrass::ffi::libc::c_int {
			use crate::{gmod13_open, gmod13_close};
			#body
		}
	}
}

pub fn generate_exit(func: &ItemFn) -> TokenStream {
	let ident = format_ident!("{}", func.sig.ident.to_string());
	let tokens = generate_func(func);
	let mut errors = Vec::new();

	for param in &func.sig.generics.params {
		if let GenericParam::Type(param) = param {
			let err = Error::new(
				param.span(),
				"the exit function cannot have type parameters",
			);
			errors.push(err.to_compile_error());
		} else if let GenericParam::Const(param) = param {
			let err = Error::new(
				param.span(),
				"the exit function cannot have const parameters",
			);
			errors.push(err.to_compile_error());
		}
	}

	let body = match errors.is_empty() {
		false => quote! { 0 },
		true => quote! {
			if ::flatgrass::lua::Lua::enter(__fg_state, |__fg_lua| {
				let __fg_func = ::flatgrass::lua::Function::new(::flatgrass::cfunction!(#ident));
				let __fg_res = ::flatgrass::lua::call!(__fg_func);
				__fg_lua.__fg_exit();
				match __fg_res {
					Ok(_) => false,
					Err(__fg_err) => {
						__fg_lua.stack().clear();
						__fg_lua.stack().push_any(__fg_err);
						true
					},
				}
			}) {
				::flatgrass::ffi::lua_error(__fg_state)
			} else {
				0
			}
		},
	};

	quote! {
		#tokens

		#(#errors)*

		#[doc(hidden)]
		#[unsafe(no_mangle)]
		pub unsafe extern "C-unwind" fn gmod13_close(__fg_state: *mut ::flatgrass::ffi::lua_State) -> ::flatgrass::ffi::libc::c_int {
			use crate::{gmod13_open, gmod13_close};
			#body
		}
	}
}

pub fn generate_func(func: &ItemFn) -> TokenStream {
	let (impl_generics, type_generics, where_clause) = func.sig.generics.split_for_impl();
	let generics_turbofish = type_generics.as_turbofish();
	let ident = format_ident!("{}", func.sig.ident.to_string());
	let vis = &func.vis;
	let mut errors = Vec::new();

	if let Some(unsafety) = &func.sig.unsafety {
		let err = Error::new(unsafety.span(), "Lua functions cannot be unsafe");
		errors.push(err.to_compile_error());
	}

	if let Some(asyncness) = &func.sig.asyncness {
		if cfg!(not(feature = "async")) {
			let err = Error::new(
				asyncness.span(),
				"async Lua functions require the `async` feature",
			);
			errors.push(err.to_compile_error());
		}
	}

	let body = match errors.is_empty() {
		false => quote! { 0 },
		true => {
			let args = func.sig.inputs.iter().map(|_| {
				quote! {
					match ::flatgrass::lua::FnParam::fn_param(__fg_lua, &mut __fg_arg, &mut __fg_upv) {
						::core::result::Result::Ok(__fg_value) => __fg_value,
						::core::result::Result::Err(__fg_err) => {
							__fg_lua.stack().clear();
							__fg_lua.stack().push_any(__fg_err);
							return ::core::option::Option::None;
						}
					}
				}
			});

			let call = match &func.sig.asyncness {
				Some(_) => quote! {
					__fg_lua.async_runtime().spawn(#ident #generics_turbofish (#(#args),*)).detach();
					::core::option::Option::Some(::flatgrass::lua::util::Return::Values(0))
				},
				None => quote! {
					match ::flatgrass::lua::FnReturn::fn_return(#ident #generics_turbofish (#(#args),*), __fg_lua) {
						::core::result::Result::Ok(::flatgrass::lua::util::Return::Values(values)) =>
							::core::option::Option::Some(::flatgrass::lua::util::Return::Values(__fg_lua.stack().push_many(values))),
						::core::result::Result::Ok(::flatgrass::lua::util::Return::Yield(values)) =>
							::core::option::Option::Some(::flatgrass::lua::util::Return::Yield(__fg_lua.stack().push_many(values))),
						::core::result::Result::Err(__fg_err) => {
							__fg_lua.stack().clear();
							__fg_lua.stack().push_any(__fg_err);
							::core::option::Option::None
						}
					}
				},
			};

			quote! {
				match ::flatgrass::lua::Lua::enter(__fg_state, |__fg_lua| {
					let (mut __fg_arg, mut __fg_upv) = (1, 1);
					#call
				}) {
					::core::option::Option::None => ::flatgrass::ffi::lua_error(__fg_state),
					::core::option::Option::Some(__fg_ret) => match __fg_ret {
						::flatgrass::lua::util::Return::Values(__fg_n) => __fg_n,
						::flatgrass::lua::util::Return::Yield(__fg_n) => {
							::flatgrass::ffi::lua_yield(__fg_state, __fg_n)
						}
					}
				}
			}
		}
	};

	quote! {
		#func

		#(#errors)*

		#[doc(hidden)]
		#[doc = "Generated by the `#[flatgrass::function]` attribute macro."]
		#vis enum #ident {}

		impl #ident {

			#[inline]
			#[doc(hidden)]
			#[doc = ::core::concat!("Returns a raw Lua function containing glue code to call the `", ::core::stringify!(#ident), "` function from Lua.")]
			pub const fn cfunction #impl_generics () -> ::flatgrass::ffi::lua_CFunction #where_clause {
				pub unsafe extern "C-unwind" fn __fg_func #impl_generics (__fg_state: *mut ::flatgrass::ffi::lua_State) -> ::flatgrass::ffi::libc::c_int #where_clause {
					#body
				}

				__fg_func #generics_turbofish
			}
		}
	}
}
