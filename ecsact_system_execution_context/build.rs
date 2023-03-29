extern crate bindgen;

use ecsact_env::sdk_include_dir;

fn ecsact_wrapper_h() -> String {
	std::env::var("ECSACT_RUST_WRAPPER_H")
		.unwrap_or("src/wrapper.h".to_string())
		.into()
}

fn main() {
	let wrapper_h = ecsact_wrapper_h();

	println!("cargo:rerun-if-changed={}", wrapper_h);

	let include_dir = sdk_include_dir();

	let bindings = bindgen::builder()
		.allowlist_var("")
		.allowlist_type("")
		.allowlist_function("ecsact_system_execution_context_.*")
		.header(wrapper_h)
		.newtype_enum("ecsact_.*")
		.clang_arg(format!("-I{}", &include_dir.to_str().unwrap()))
		.clang_arg("--target=wasm32-unknown-unknown")
		.parse_callbacks(Box::new(bindgen::CargoCallbacks))
		.generate()
		.expect("Unable to generate bindings");

	bindings
		.write_to_file("src/bindings.rs")
		.expect("Couldn't write bindings!");
}
