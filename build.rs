extern crate bindgen;
extern crate cc;
use std::path::{Path, PathBuf};
use std::{env, fs};

fn main() {
	let out_dir = env::var_os("OUT_DIR").unwrap();
	println!("cargo:rerun-if-changed=build.rs");
	let dest_dir = Path::new(&out_dir).join("qulacs");
	let csim_dir = Path::new("contrib").join("qulacs").join("src").join("csim");
	// let preprocess = "_USE_SIMD";
	let _ = fs::remove_dir_all(&dest_dir);
	fs::create_dir_all(&dest_dir).unwrap();
	let mut files = Vec::new();
	for entry in fs::read_dir(&csim_dir).unwrap() {
		let entry = entry.unwrap();
		let dest_file = dest_dir.join(entry.file_name());
		fs::copy(entry.path(), &dest_file).unwrap();
		if let Some("c") = dest_file.extension().and_then(|o| o.to_str()) {
			files.push(dest_file)
		}
	}
	cc::Build::new()
		.files(files)
		.warnings(false)
		.compile("qulacs");
	// cc.define(preprocess, "1");
	let csim_dir = csim_dir.to_str().unwrap();
	let bindings = bindgen::Builder::default()
		.header(format!("{csim_dir}/update_ops.h"))
		.header(format!("{csim_dir}/stat_ops.h"))
		.header(format!("{csim_dir}/init_ops.h"))
		.parse_callbacks(Box::new(bindgen::CargoCallbacks))
		.generate()
		.expect("Unable to generate bindings");

	let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
	bindings
		.write_to_file(out_path.join("qulacs.rs"))
		.expect("Couldn't write bindings!");
}
