extern crate bindgen;
extern crate cc;
use std::env;
use std::path::PathBuf;

fn main() {
	let ops = vec!["X", "Y", "Z", "H", "CZ", "CNOT", "SWAP", "projection"];
	// let preprocess = "_USE_SIMD";
	cc::Build::new()
		.files(
			ops.iter()
				.map(|name| format!("contrib/qulacs/src/csim/update_ops_named_{}.c", name)),
		)
		.file("contrib/qulacs/src/csim/update_ops_named.c")
		.file("contrib/qulacs/src/csim/constant.c")
		.file("contrib/qulacs/src/csim/update_ops_matrix_phase_single.c")
		// .define(preprocess, "1")
		.compile("qulacs");

	let bindings = bindgen::Builder::default()
		.header("contrib/qulacs/src/csim/update_ops.h")
		.header("contrib/qulacs/src/csim/constant.h")
		.parse_callbacks(Box::new(bindgen::CargoCallbacks))
		.generate()
		.expect("Unable to generate bindings");

	let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
	bindings
		.write_to_file(out_path.join("qulacs.rs"))
		.expect("Couldn't write bindings!");
}
