fn main() {
	println!("cargo::rustc-check-cfg=cfg(fg_win32)");
	println!("cargo::rustc-check-cfg=cfg(fg_win64)");
	println!("cargo::rustc-check-cfg=cfg(fg_linux32)");
	println!("cargo::rustc-check-cfg=cfg(fg_linux64)");
	println!("cargo::rustc-check-cfg=cfg(fg_unsupported)");

	if cfg!(all(target_os = "windows", target_arch = "x86")) {
		println!("cargo:rustc-cfg=fg_win32");
	} else if cfg!(all(target_os = "windows", target_arch = "x86_64")) {
		println!("cargo:rustc-cfg=fg_win64");
	} else if cfg!(all(target_os = "linux", target_arch = "x86")) {
		println!("cargo:rustc-cfg=fg_linux32");
	} else if cfg!(all(target_os = "linux", target_arch = "x86_64")) {
		println!("cargo:rustc-cfg=fg_linux64");
	} else {
		println!("cargo:rustc-cfg=fg_unsupported");
	}
}
