use rustc_version::{version_meta, Channel};

fn main() {
	println!("cargo:rerun-if-changed=build.rs");
	println!("cargo::rustc-check-cfg=cfg(fg_nightly)");
	if let Ok(Channel::Nightly) = version_meta().map(|v| v.channel) {
		println!("cargo:rustc-cfg=fg_nightly");
	}
}
