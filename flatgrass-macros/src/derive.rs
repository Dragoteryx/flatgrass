use proc_macro2::*;
use quote::{quote, quote_spanned};
use syn::spanned::Spanned;
use syn::*;

pub fn to_lua(mut input: DeriveInput) -> TokenStream {
	match input.data {
		Data::Enum(data) => quote_spanned!(data.enum_token.span =>
			compile_error!("`ToLua` cannot be derived from enums");
		),
		Data::Union(data) => quote_spanned!(data.union_token.span =>
			compile_error!("`ToLua` cannot be derived from unions");
		),
		Data::Struct(data) => {
			for param in &mut input.generics.params {
				if let GenericParam::Type(param) = param {
					param
						.bounds
						.push(parse_quote! { ::flatgrass::lua::traits::ToLua });
				}
			}

			let (impl_generics, type_generics, where_clause) = input.generics.split_for_impl();
			let ident = &input.ident;

			let fields_to_lua_by_ref = data.fields.iter().enumerate().map(|(i, field)| match field
				.ident
				.as_ref()
			{
				Some(field_ident) => quote_spanned! { field.span() =>
					#field_ident: ::flatgrass::lua::traits::ToLua::to_lua_by_ref(&self.#field_ident)
				},
				None => {
					let field_ident = Index::from(i);
					quote_spanned! { field.span() =>
						::flatgrass::lua::traits::ToLua::to_lua_by_ref(&self.#field_ident)
					}
				}
			});

			let fields_to_lua =
				data.fields
					.iter()
					.enumerate()
					.map(|(i, field)| match field.ident.as_ref() {
						Some(field_ident) => quote_spanned! { field.span() =>
							#field_ident: ::flatgrass::lua::traits::ToLua::to_lua(self.#field_ident)
						},
						None => {
							let field_ident = Index::from(i);
							quote_spanned! { field.span() =>
								::flatgrass::lua::traits::ToLua::to_lua(self.#field_ident)
							}
						}
					});

			quote! {
				#[automatically_derived]
				impl #impl_generics ::flatgrass::lua::traits::ToLua for #ident #type_generics #where_clause {
					fn to_lua_by_ref(&self) -> ::flatgrass::lua::value::LuaValue {
						::flatgrass::lua::table! { #(#fields_to_lua_by_ref),* }.to_lua()
					}

					fn to_lua(self) -> ::flatgrass::lua::value::LuaValue {
						::flatgrass::lua::table! { #(#fields_to_lua),* }.to_lua()
					}
				}
			}
		}
	}
}
