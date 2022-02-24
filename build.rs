extern crate bindgen;
extern crate cc;
use std::path::{Path, PathBuf};
use std::{env, fs};

fn main() {
	let out_dir = env::var_os("OUT_DIR").unwrap();
	println!("cargo:rerun-if-changed=build.rs");
	let dest_dir = Path::new(&out_dir).join("qulacs");
	let qulacs_dir = Path::new("contrib").join("qulacs");
	let csim_dir = qulacs_dir.join("src").join("csim");
	// let preprocess = "_USE_SIMD";
	let _ = fs::remove_dir_all(&dest_dir);
	fs::create_dir_all(&dest_dir).unwrap();

	let mut files = Vec::new();
	for entry in fs::read_dir(csim_dir).unwrap() {
		let entry = entry.unwrap();
		let dest_file = dest_dir.join(entry.file_name());
		fs::copy(entry.path(), &dest_file).unwrap();
		match dest_file.extension().and_then(|o| o.to_str()) {
			Some("c") => files.push(dest_file),
			_ => (),
		}
	}
	cc::Build::new().files(files).compile("qulacs");
	// cc.define(preprocess, "1");
	let bindings = bindgen::Builder::default()
		.header("contrib/qulacs/src/csim/update_ops.h")
		.parse_callbacks(Box::new(bindgen::CargoCallbacks))
		.generate()
		.expect("Unable to generate bindings");

	let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
	bindings
		.write_to_file(out_path.join("qulacs.rs"))
		.expect("Couldn't write bindings!");
}
