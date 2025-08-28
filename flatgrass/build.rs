use rustc_version::Channel;
use rustc_version::version_meta;

pub fn main() {
	println!("cargo::rustc-check-cfg=cfg(fg_nightly)");
	if version_meta().unwrap().channel == Channel::Nightly {
		println!("cargo:rustc-cfg=fg_nightly");
	}
}
