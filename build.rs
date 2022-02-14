extern crate bindgen;
extern crate cc;
use std::env;
use std::path::PathBuf;

fn main() {
	cc::Build::new()
		.file("cpp/operator.cpp")
		.compile("operator");

	let bindings = bindgen::Builder::default()
		.header("cpp/operator.hpp")
		.parse_callbacks(Box::new(bindgen::CargoCallbacks))
		.generate()
		.expect("Unable to generate bindings");

	let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
	bindings
		.write_to_file(out_path.join("operator.rs"))
		.expect("Couldn't write bindings!");
}
