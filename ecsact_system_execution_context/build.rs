extern crate bindgen;

// TODO(zaucy): Optionally include runfiles only while compiling with bazel
use runfiles::Runfiles;
use std::process::Command;

fn ecsact_include_dir() -> String {
	// This environment variable is really only for the bazel build. Users should
	// just use the `ecsact` command line in their PATH
	let rt_headers = std::env::var("ECSACT_RUNTIME_HEADERS");
	if rt_headers.is_ok() {
		let runfiles = Runfiles::create().unwrap();
		let header = runfiles
			.rlocation("ecsact_runtime/ecsact/runtime.h")
			.into_os_string()
			.into_string()
			.unwrap()
			.replace("\\", "/");
		let header_index = header.find("/ecsact/").unwrap();
		let include_dir = &header[..header_index];
		return include_dir.into();
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

fn ecsact_wrapper_h() -> String {
	std::env::var("ECSACT_RUST_WRAPPER_H")
		.unwrap_or("src/wrapper.h".to_string())
		.into()
}

fn main() {
	let wrapper_h = ecsact_wrapper_h();

	println!("cargo:rerun-if-changed={}", wrapper_h);

	let include_dir = ecsact_include_dir();

	let bindings = bindgen::builder()
		.allowlist_var("")
		.allowlist_type("")
		.allowlist_function("ecsact_system_execution_context_.*")
		.header(wrapper_h)
		.newtype_enum("ecsact_.*")
		.clang_arg("-I".to_string() + &include_dir)
		.clang_arg("--target=wasm32-unknown-unknown")
		.parse_callbacks(Box::new(bindgen::CargoCallbacks))
		.generate()
		.expect("Unable to generate bindings");

	bindings
		.write_to_file("src/bindings.rs")
		.expect("Couldn't write bindings!");
}
