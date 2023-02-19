extern crate bindgen;

use std::env;
use std::path::PathBuf;
use std::process::Command;

fn ecsact_include_dir() -> String {
	// This environment variable is really only for the bazel build. Users should
	// just use the `ecsact` command line in their PATH
	let rt_headers = env::var("ECSACT_RUNTIME_HEADERS");
	if let Ok(rt_headers) = rt_headers {
		let rt_headers: Vec<&str> = rt_headers.split(' ').collect();
		let header = rt_headers.first().unwrap().to_owned().replace("\\", "/");
		let header_index = header.find("/ecsact/").unwrap();
		let include_dir = &header[..header_index];
		return "../".to_string() + include_dir.into();
	}

	let ecsact_config = json::parse(&String::from_utf8_lossy(
		&Command::new("ecsact")
			.arg("config")
			.output()
			.expect("ecsact config failed")
			.stdout,
	))
	.unwrap();

	return ecsact_config["include_dir"].as_str().unwrap().into();
}

fn ecsact_dylib_cc_path() -> String {
	env::var("ECSACT_RUST_DYLIB_CC")
		.unwrap_or("src/dylib.cc".to_string())
		.into()
}

fn ecsact_dylib_wrapper_h() -> String {
	env::var("ECSACT_RUST_DYLIB_WRAPPER_H")
		.unwrap_or("src/dylib_wrapper.h".to_string())
		.into()
}

fn main() {
	let dylib_cc_path = ecsact_dylib_cc_path();
	let dylib_wrapper_h_path = ecsact_dylib_wrapper_h();

	println!("cargo:rerun-if-changed={}", dylib_wrapper_h_path);
	println!("cargo:rerun-if-changed={}", dylib_cc_path);

	let include_dir = ecsact_include_dir();

	dbg!(&include_dir);
	dbg!(std::env::current_dir()
		.unwrap()
		.to_str()
		.unwrap()
		.replace("\\", "/"));

	cc::Build::new()
		.cpp(true)
		.file(dylib_cc_path)
		.include(&include_dir)
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
		.allowlist_var("ecsact_.*")
		.allowlist_type("ecsact_.*")
		.allowlist_function("ecsact_.*")
		.newtype_enum("ecsact_.*")
		.clang_arg("-I".to_string() + &include_dir)
		.header(dylib_wrapper_h_path)
		.parse_callbacks(Box::new(bindgen::CargoCallbacks))
		.generate()
		.expect("Unable to generate bindings");

	let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
	bindings
		.write_to_file(out_path.join("bindings.rs"))
		.expect("Couldn't write bindings!");
}
