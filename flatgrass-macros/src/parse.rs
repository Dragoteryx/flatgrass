use syn::AngleBracketedGenericArguments;
use syn::parse::{Parse, ParseStream};
use syn::spanned::Spanned;

#[repr(transparent)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LuaFn(pub syn::ItemFn);

#[repr(transparent)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EntryFn(pub LuaFn);

#[repr(transparent)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ExitFn(pub LuaFn);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LuaFnDecl {
	pub turbofish: Option<syn::AngleBracketedGenericArguments>,
	pub ident: syn::Ident,
}

impl Parse for LuaFn {
	fn parse(input: ParseStream) -> syn::Result<Self> {
		let item_fn: syn::ItemFn = input.parse()?;
		let ident = item_fn.sig.ident.to_string();
		let mut err = None::<syn::Error>;
		let mut combine_err = |span, msg| {
			let new_err = syn::Error::new(span, msg);
			if let Some(err) = &mut err {
				err.combine(new_err);
			} else {
				err = Some(new_err);
			}
		};

		#[cfg(not(feature = "async"))]
		if let Some(asyncness) = &item_fn.sig.asyncness {
			combine_err(
				asyncness.span(),
				"async Lua functions require the `async` feature",
			);
		}

		if let syn::Safety::Unsafe(unsafety) = &item_fn.sig.safety {
			combine_err(unsafety.span(), "Lua functions cannot be unsafe");
		}

		if ident == "gmod13_open" {
			combine_err(
				item_fn.sig.ident.span(),
				"the name `gmod13_open` is reserved",
			);
		}

		if ident == "gmod13_close" {
			combine_err(
				item_fn.sig.ident.span(),
				"the name `gmod13_close` is reserved",
			);
		}

		match err {
			None => Ok(LuaFn(item_fn)),
			Some(err) => Err(err),
		}
	}
}

impl Parse for EntryFn {
	fn parse(input: ParseStream) -> syn::Result<Self> {
		let lua_fn: LuaFn = input.parse()?;
		let mut err = None::<syn::Error>;
		let mut combine_err = |span, msg| {
			let new_err = syn::Error::new(span, msg);
			if let Some(err) = &mut err {
				err.combine(new_err);
			} else {
				err = Some(new_err);
			}
		};

		for param in &lua_fn.0.sig.generics.params {
			if let syn::GenericParam::Type(param) = param {
				combine_err(
					param.span(),
					"the entry function cannot have type parameters",
				);
			} else if let syn::GenericParam::Const(param) = param {
				combine_err(
					param.span(),
					"the entry function cannot have const parameters",
				);
			}
		}

		match err {
			None => Ok(EntryFn(lua_fn)),
			Some(err) => Err(err),
		}
	}
}

impl Parse for ExitFn {
	fn parse(input: ParseStream) -> syn::Result<Self> {
		let lua_fn: LuaFn = input.parse()?;
		let mut err = None::<syn::Error>;
		let mut combine_err = |span, msg| {
			let new_err = syn::Error::new(span, msg);
			if let Some(err) = &mut err {
				err.combine(new_err);
			} else {
				err = Some(new_err);
			}
		};

		for param in &lua_fn.0.sig.generics.params {
			if let syn::GenericParam::Type(param) = param {
				combine_err(
					param.span(),
					"the exit function cannot have type parameters",
				);
			} else if let syn::GenericParam::Const(param) = param {
				combine_err(
					param.span(),
					"the exit function cannot have const parameters",
				);
			}
		}

		match err {
			None => Ok(ExitFn(lua_fn)),
			Some(err) => Err(err),
		}
	}
}

impl Parse for LuaFnDecl {
	fn parse(input: ParseStream) -> syn::Result<Self> {
		let ident = input.parse()?;
		let turbofish = if input.peek(syn::Token![::]) {
			Some(AngleBracketedGenericArguments::parse_turbofish(input)?)
		} else {
			None
		};

		Ok(LuaFnDecl { turbofish, ident })
	}
}
