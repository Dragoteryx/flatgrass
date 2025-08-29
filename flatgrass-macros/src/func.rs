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
			if ::flatgrass::lua::Lua::init(__fg_state, |__fg_lua| {
				let __fg_func = ::flatgrass::lua::func!(#ident);
				__fg_lua.__fg_entry();
				match __fg_func.call(()) {
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

		#(#errors;)*

		#[doc(hidden)]
		#[unsafe(no_mangle)]
		pub unsafe extern "C-unwind" fn gmod13_open(__fg_state: *mut ::flatgrass::ffi::lua_State) -> ::flatgrass::ffi::libc::c_int {
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
			if ::flatgrass::lua::Lua::init(__fg_state, |__fg_lua| {
				let __fg_func = ::flatgrass::lua::func!(#ident);
				let __fg_res = __fg_func.call(());
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

		#(#errors;)*

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
	let ident = &func.sig.ident;
	let vis = &func.vis;
	let mut errors = Vec::new();

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
		false => quote! { 0 },
		true => {
			let args = func.sig.inputs.iter().map(|input| {
				quote_spanned! { input.span() =>
					match ::flatgrass::lua::traits::LuaFnParam::lua_fn_param(__fg_lua, &mut __fg_arg, &mut __fg_upv) {
						::core::result::Result::Ok(__fg_value) => __fg_value,
						::core::result::Result::Err(__fg_err) => {
							__fg_lua.stack().clear();
							__fg_lua.stack().push_any(__fg_err);
							return ::core::option::Option::None;
						}
					}
				}
			});

			let ret_span = match &func.sig.output {
				ReturnType::Default => func.sig.output.span(),
				ReturnType::Type(_, ty) => ty.span(),
			};

			let call = match &func.sig.asyncness {
				Some(_) => quote_spanned! { ret_span => },
				None => quote_spanned! { ret_span =>
					match ::flatgrass::lua::traits::LuaFnReturn::lua_fn_return(#ident #generics_turbofish (#(#args),*), __fg_lua) {
						::core::result::Result::Ok(::flatgrass::lua::traits::Return::Values(values)) =>
							::core::option::Option::Some(::flatgrass::lua::traits::Return::Values(__fg_lua.stack().push_many(values))),
						::core::result::Result::Ok(::flatgrass::lua::traits::Return::Yield(values)) =>
							::core::option::Option::Some(::flatgrass::lua::traits::Return::Yield(__fg_lua.stack().push_many(values))),
						::core::result::Result::Err(__fg_err) => {
							__fg_lua.stack().clear();
							__fg_lua.stack().push_any(__fg_err);
							::core::option::Option::None
						}
					}
				},
			};

			quote! {
				match ::flatgrass::lua::Lua::init(__fg_state, |__fg_lua| {
					let (mut __fg_arg, mut __fg_upv) = (1, 1);
					#call
				}) {
					::core::option::Option::None => ::flatgrass::ffi::lua_error(__fg_state),
					::core::option::Option::Some(__fg_ret) => match __fg_ret {
						::flatgrass::lua::traits::Return::Values(__fg_i) => __fg_i,
						::flatgrass::lua::traits::Return::Yield(__fg_i) => {
							::flatgrass::ffi::lua_yield(__fg_state, __fg_i)
						}
					}
				}
			}
		}
	};

	quote! {
		#func

		#(#errors;)*

		#[doc(hidden)]
		#[doc = "Generated by the `#[flatgrass::function]` attribute macro."]
		#vis mod #ident {
			use super::*;

			#[inline]
			#[doc = ::core::concat!("Returns a raw Lua function containing glue code to call the `", ::core::stringify!(#ident), "` function from Lua.")]
			pub const fn to_lua #impl_generics () -> ::flatgrass::ffi::lua_CFunction #where_clause {
				pub unsafe extern "C-unwind" fn __fg_func #impl_generics (__fg_state: *mut ::flatgrass::ffi::lua_State) -> ::flatgrass::ffi::libc::c_int #where_clause {
					#body
				}

				__fg_func::<#type_generics>
			}
		}
	}
}
