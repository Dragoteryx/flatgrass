use super::*;

fn check_constness(item: &syn::ItemFn) -> Option<TokenStream2> {
  item.sig.constness.map(|constness| quote_spanned! {
    constness.span() => compile_error!("lua functions cannot be const");
  })
}

fn check_asyncness(item: &syn::ItemFn) -> Option<TokenStream2> {
  item.sig.asyncness.map(|asyncness| quote_spanned! {
    asyncness.span() => compile_error!("lua functions cannot be async");
  })
}

fn check_abi(item: &syn::ItemFn) -> Option<TokenStream2> {
  item.sig.abi.as_ref().map(|abi| quote_spanned! {
    abi.span() => compile_error!("lua functions cannot specify an ABI");
  })
}

fn check_self_param(item: &syn::ItemFn) -> Option<TokenStream2> {
  item.sig.inputs.iter().find_map(|input| match input {
    syn::FnArg::Typed(_) => None,
    syn::FnArg::Receiver(receiver) => Some(quote_spanned! {
      receiver.span() => compile_error!("lua functions cannot have a `self` parameter");
    })
  })
}

pub fn check_valid(item: &syn::ItemFn) -> Option<TokenStream2> {
  check_constness(item)
    .or_else(|| check_asyncness(item))
    .or_else(|| check_abi(item))
    .or_else(|| check_self_param(item))
}

macro_rules! check_name {
  ($item:expr, $name:literal) => {
    ($item.sig.ident != $name).then(|| quote_spanned! {
      $item.sig.ident.span() => compile_error!(concat!("this function needs to be named `", $name, "`"));
    })
  }
}

pub fn wrap_function(mut item: syn::ItemFn) -> TokenStream2 {
  let mut ident = format_ident!("__fg_{}", item.sig.ident);
  std::mem::swap(&mut ident, &mut item.sig.ident);
  let wrapped_ident = &item.sig.ident;

  let vis = item.vis;
  item.vis = syn::Visibility::Inherited;

  let args = item.sig.inputs.iter().map(|_| quote! {
    ::flatgrass::lua::traits::LuaParam::resolve(state, &mut idx).unwrap_or_else(|err| {
      ::flatgrass::lua::traits::ToLua::push(state, err);
      ::flatgrass::ffi::lua_error(state);
      ::std::unreachable!()
    })
  });

  match item.sig.output {
    syn::ReturnType::Default => quote! {
      #[no_mangle]
      #vis unsafe extern "C-unwind" fn #ident(state: ::flatgrass::ffi::LuaState) -> i32 {
        #item
        
        let mut idx = 1;
        #wrapped_ident(#(#args),*);
        0
      }
    },
    _ => quote! {
      #[no_mangle]
      #vis unsafe extern "C-unwind" fn #ident(state: ::flatgrass::ffi::LuaState) -> i32 {
        #item
        
        let mut idx = 1;
        let ret = #wrapped_ident(#(#args),*);
        let top = ::flatgrass::ffi::lua_gettop(state);
        ::flatgrass::lua::traits::LuaReturn::push(state, ret);
        (::flatgrass::ffi::lua_gettop(state) - top).max(0)
      }
    }
  }
}