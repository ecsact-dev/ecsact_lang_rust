extern crate bindgen;

use std::process::Command;

fn main() {
	println!("cargo:rerun-if-changed=src/wrapper.h");

	let ecsact_config = json::parse(&String::from_utf8_lossy(
		&Command::new("ecsact")
			.arg("config")
			.output()
			.expect("ecsact config failed")
			.stdout,
	))
	.unwrap();

	let include_dir = ecsact_config["include_dir"].as_str().unwrap();

	let bindings = bindgen::builder()
		.allowlist_var("ecsact_.*")
		.allowlist_type("ecsact_.*")
		.allowlist_function("ecsact_.*")
		.newtype_enum("ecsact_.*")
		.clang_arg("-I".to_string() + include_dir)
		.header("src/wrapper.h")
		.parse_callbacks(Box::new(bindgen::CargoCallbacks))
		.generate()
		.expect("Unable to generate bindings");

	bindings
		.write_to_file("src/bindings.generated.rs")
		.expect("Couldn't write bindings!");
}
