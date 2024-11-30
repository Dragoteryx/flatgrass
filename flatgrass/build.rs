use std::env;

fn main() {
	println!("cargo::rustc-check-cfg=cfg(fg_win32)");
	println!("cargo::rustc-check-cfg=cfg(fg_win64)");
	println!("cargo::rustc-check-cfg=cfg(fg_linux32)");
	println!("cargo::rustc-check-cfg=cfg(fg_linux64)");
	println!("cargo::rustc-check-cfg=cfg(fg_unsupported)");

	let os = env::var("CARGO_CFG_TARGET_OS")
		.expect("CARGO_CFG_TARGET_OS not set");
	let arch = env::var("CARGO_CFG_TARGET_ARCH")
		.expect("CARGO_CFG_TARGET_ARCH not set");
	
	match (os.as_str(), arch.as_str()) {
		("windows", "x86") => println!("cargo:rustc-cfg=fg_win32"),
		("windows", "x86_64") => println!("cargo:rustc-cfg=fg_win64"),
		("linux", "x86") => println!("cargo:rustc-cfg=fg_linux32"),
		("linux", "x86_64") => println!("cargo:rustc-cfg=fg_linux64"),
		_ => println!("cargo:rustc-cfg=fg_unsupported"),
	}
}
