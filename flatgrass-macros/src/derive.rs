use proc_macro2::*;
use quote::{quote, quote_spanned};
use syn::spanned::Spanned;
use syn::*;

pub fn to_lua(mut input: DeriveInput) -> TokenStream {
	for param in &mut input.generics.params {
		if let GenericParam::Type(param) = param {
			param.bounds.push(parse_quote!(::flatgrass::lua::traits::ToLua));
		}
	}

	match &input.data {
		Data::Union(_) => quote! {
			compile_error!("ToLua cannot be derived from unions");
		},
		Data::Enum(_) => quote! {
			compile_error!("ToLua cannot be derived from enums");
		},
		Data::Struct(data) => {
			let ident = &input.ident;
			let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
			match &data.fields {
				Fields::Unit => quote! { },
				Fields::Unnamed(fields) => {
					let fields_to_lua = fields.unnamed.iter().enumerate().map(|(i, field)| {
						let index = Index::from(i);
						quote_spanned! { field.span() =>
							::flatgrass::lua::traits::ToLua::to_lua(self.#index)
						}
					});

					let fields_to_lua_by_ref = fields.unnamed.iter().enumerate().map(|(i, field)| {
						let index = Index::from(i);
						quote_spanned! { field.span() =>
							::flatgrass::lua::traits::ToLua::to_lua_by_ref(&self.#index)
						}
					});

					quote! {
						#[automatically_derived]
						impl #impl_generics ::flatgrass::lua::traits::ToLua for #ident #ty_generics #where_clause {
							fn to_lua_by_ref(&self) -> ::flatgrass::lua::value::LuaValue {
								::flatgrass::lua::value::LuaValue::Table(::flatgrass::lua::table![
									#(#fields_to_lua_by_ref,)*
								])
							}
				
							fn to_lua(self) -> ::flatgrass::lua::value::LuaValue
							where
								Self: Sized,
							{
								::flatgrass::lua::value::LuaValue::Table(::flatgrass::lua::table![
									#(#fields_to_lua,)*
								])
							}
						}
					}
				}
				Fields::Named(fields) => {
					let fields_to_lua = fields.named.iter().map(|field| {
						let name = field.ident.as_ref().unwrap();
						quote_spanned! { field.span() =>
							#name: ::flatgrass::lua::traits::ToLua::to_lua(self.#name)
						}
					});

					let fields_to_lua_by_ref = fields.named.iter().map(|field| {
						let name = field.ident.as_ref().unwrap();
						quote_spanned! { field.span() =>
							#name: ::flatgrass::lua::traits::ToLua::to_lua_by_ref(&self.#name)
						}
					});
					
					quote! {
						#[automatically_derived]
						impl #impl_generics ::flatgrass::lua::traits::ToLua for #ident #ty_generics #where_clause {
							fn to_lua_by_ref(&self) -> ::flatgrass::lua::value::LuaValue {
								::flatgrass::lua::value::LuaValue::Table(::flatgrass::lua::table! {
									#(#fields_to_lua_by_ref,)*
								})
							}
				
							fn to_lua(self) -> ::flatgrass::lua::value::LuaValue
							where
								Self: Sized,
							{
								::flatgrass::lua::value::LuaValue::Table(::flatgrass::lua::table! {
									#(#fields_to_lua,)*
								})
							}
						}
					}
				}
			}
		}
	}
}
