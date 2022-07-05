use super::*;

macro_rules! spanned {
  ($span:expr, $exp:expr) => {
    quote::quote_spanned!(
      $span.span() => $exp
    )
  };
}

macro_rules! spanned_error {
  ($span:expr, $err:expr) => {
    spanned!($span, compile_error!($err))
  };
}

fn check_asyncness(func: &syn::ItemFn) -> Option<TokenStream2> {
  func.sig.asyncness
    .map(|asyncness| spanned_error!(asyncness, "lua functions cannot be async"))
}

fn check_generics(func: &syn::ItemFn) -> Option<TokenStream2> {
  (!func.sig.generics.params.is_empty())
    .then(|| spanned_error!(func.sig.generics, "lua functions cannot have generic parameters"))
}

fn check_where(func: &syn::ItemFn) -> Option<TokenStream2> {
  func.sig.generics.where_clause.as_ref()
    .map(|where_clause| spanned_error!(where_clause, "lua functions cannot have where clauses"))
}

fn check_self(func: &syn::ItemFn) -> Option<TokenStream2> {
  func.sig.inputs.iter().find_map(|arg| match arg {
    syn::FnArg::Receiver(receiver) => Some(spanned_error!(receiver, "lua functions cannot have a `self` parameter")),
    syn::FnArg::Typed(_) => None
  })
}

pub fn check_valid(func: &syn::ItemFn) -> Option<TokenStream2> {
 check_asyncness(func)
    .or_else(|| check_generics(func))
    .or_else(|| check_where(func))
    .or_else(|| check_self(func))
}

pub fn gen_function(lua_ident: syn::Ident, func: syn::ItemFn) -> TokenStream2 {
  let (func_ident, vis) = (&func.sig.ident, &func.vis);
  let args = func.sig.inputs.iter().map(|arg| spanned!(
    arg, match LuaArg::resolve(state, &mut narg) {
      Err(err) => return Err(format!("{}", err)),
      Ok(value) => value
    }
  ));

  match &func.sig.output {
    syn::ReturnType::Type(_, typ) => {
      let ret = spanned!(typ, LuaReturn::push(state, ret).map_err(|err| format!("{}", err)));

      quote::quote!(
        #func
  
        #[no_mangle]
        #[allow(clippy::useless_format)]
        #vis unsafe extern "C-unwind" fn #lua_ident(state: ::flatgrass::ffi::LuaState) -> i32 {
          use ::flatgrass::lua::traits::{LuaArg, LuaReturn};

          let run = || -> Result<i32, String> {
            let mut narg = 1;
            let ret = #func_ident(#(#args),*);
            #ret
          };
  
          match run() {
            Ok(ret) => ret,
            Err(err) => {
              state.fg_checkstack(1);
              state.fg_pushvalue(err);
              state.lua_error();
              0
            }
          }
        }
      )
    }
    _ => quote::quote!(
      #func

      #[no_mangle]
      #[allow(clippy::useless_format)]
      #vis unsafe extern "C-unwind" fn #lua_ident(state: ::flatgrass::ffi::LuaState) -> i32 {
        use ::flatgrass::lua::traits::LuaArg;

        let run = || -> Result<i32, String> {
          let mut narg = 1;
          #func_ident(#(#args),*);
          Ok(0)
        };

        match run() {
          Ok(ret) => ret,
          Err(err) => {
            state.fg_checkstack(1);
            state.fg_pushvalue(err);
            state.lua_error();
            0
          }
        }
      }
    )
  }
}