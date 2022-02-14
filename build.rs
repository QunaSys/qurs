extern crate bindgen;
extern crate cc;
use std::env;
use std::path::PathBuf;

fn main() {
	let ops = vec!["X", "Y", "Z"];
	cc::Build::new()
		.files(
			ops.iter()
				.map(|name| format!("contrib/qulacs/src/csim/update_ops_named_{}.c", name)),
		)
		.compile("qulacs");

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
