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