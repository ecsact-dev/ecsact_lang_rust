extern crate bindgen;

use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
	// Tell cargo to look for shared libraries in the specified directory
	// println!("cargo:rustc-link-search=/path/to/lib");

	// Tell cargo to tell rustc to link the system bzip2
	// shared library.
	// println!("cargo:rustc-link-lib=bz2");

	// Tell cargo to invalidate the built crate whenever the wrapper changes
	println!("cargo:rerun-if-changed=src/dylib_wrapper.h");
	println!("cargo:rerun-if-changed=src/dylib.cc");

	let ecsact_config = json::parse(&String::from_utf8_lossy(
		&Command::new("ecsact")
			.arg("config")
			.output()
			.expect("ecsact config failed")
			.stdout,
	))
	.unwrap();

	let include_dir = ecsact_config["include_dir"].as_str().unwrap();

	cc::Build::new()
		.cpp(true)
		.file("src/dylib.cc")
		.include(include_dir)
		.define("ECSACT_ASYNC_API_LOAD_AT_RUNTIME", "")
		.define("ECSACT_CORE_API_LOAD_AT_RUNTIME", "")
		.define("ECSACT_DYNAMIC_API_LOAD_AT_RUNTIME", "")
		.define("ECSACT_META_API_LOAD_AT_RUNTIME", "")
		.define("ECSACT_SERIALIZE_API_LOAD_AT_RUNTIME", "")
		.flag_if_supported("-std=c++20")
		.flag_if_supported("/std:c++20")
		.flag_if_supported("/permissive-")
		.flag_if_supported("/Zc:preprocessor")
		.compile("ecsact_dylib_runtime");

	let bindings = bindgen::Builder::default()
		.whitelist_var("ecsact_.*")
		.whitelist_type("ecsact_.*")
		.whitelist_function("ecsact_.*")
		.newtype_enum("ecsact_.*")
		.clang_arg("-I".to_string() + include_dir)
		.header("src/dylib_wrapper.h")
		.parse_callbacks(Box::new(bindgen::CargoCallbacks))
		.generate()
		.expect("Unable to generate bindings");

	let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
	bindings
		.write_to_file(out_path.join("bindings.rs"))
		.expect("Couldn't write bindings!");
}
