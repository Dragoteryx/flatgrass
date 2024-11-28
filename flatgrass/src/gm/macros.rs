/// Prints to the Garry's Mod console using the [`print`](https://wiki.facepunch.com/gmod/Global.print) function.
#[doc(hidden)]
#[macro_export]
macro_rules! printfg {
	($($arg:expr),+) => {
		$crate::gm::print(format!($($arg),*))
	};
}
