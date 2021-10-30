extern crate bindgen;
extern crate cc;

use std::env;
use std::ffi::OsString;
use std::fs;
use std::io;
use std::io::{BufRead, Write};
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;

fn build_qulacs(out_dir: &OsStr) {
	let dest_dir = Path::new(out_dir).join("qulacs");
	let qulacs_dir = Path::new("contrib").join("qulacs");
	let csim_dir = qulacs_dir.join("src").join("csim");

	// mkdir $OUT_DIR/qulacs
	let _ = fs::remove_dir_all(&dest_dir);
	fs::create_dir_all(&dest_dir).unwrap();

	let mut files = Vec::new();
	for entry in fs::read_dir(csim_dir).unwrap() {
		let entry = entry.unwrap();
		let dest_file = dest_dir.join(entry.file_name());
		println!("cargo:rerun-if-changed={}", entry.path().to_str().unwrap());
		fs::copy(entry.path(), &dest_file).unwrap();
		match dest_file.extension().and_then(|o| o.to_str()) {
			Some("cc") | Some("c") => {
				if dest_file
					.iter()
					.last()
					.unwrap()
					.to_str()
					.unwrap()
					.starts_with("update_ops_named_")
				{
					files.push(dest_file)
				}
			}
			_ => (),
		}
	}

	let mut cc = cc::Build::new();
	let mut cc = files
		.iter()
		.fold(&mut cc, |cc, file| cc.file(file.to_str().unwrap()));
	// if cfg!(windows) {
	//     cc = cc.define("WIN", "true");
	// }
	cc.warnings(false)
		.extra_warnings(false)
		.opt_level(3)
		.flag("-std=c99")
		//     .flag("-xc")
		//     .flag("-lm")
		.shared_flag(true)
		.static_flag(true)
		.out_dir(&dest_dir)
		.cargo_metadata(true)
		.compile("qulacs");
}

fn generate_binding(out_dir: &OsStr) {
	let bindings = bindgen::Builder::default()
		.no_unstable_rust()
		.header("wrapper.h")
		.generate()
		.expect("Unable to generate bindings");

	// Write the bindings to the $OUT_DIR/bindings.rs file.
	let out_path = PathBuf::from(out_dir);
	bindings
		.write_to_file(out_path.join("bindings.rs"))
		.expect("Couldn't write bindings!");
}

fn main() {
	let out_dir = env::var_os("OUT_DIR").unwrap();
	println!("cargo:rerun-if-changed=build.rs");
}
