use std::fs::File;
use std::process::Command;

fn main() {
	// TODO(zaucy): Figure out how to get this path correctly
	let rust_plugin_path = "../target/debug/ecsact_rust_source_codegen.dll";
	let ecsact_src_path = "src/example.ecsact";

	println!("cargo:rerun-if-changed={}", ecsact_src_path);
	println!("cargo:rerun-if-changed={}", rust_plugin_path);

	let ecsact_rs_file = File::create("src/example_ecsact.rs").unwrap();

	let codegen_output = Command::new("ecsact")
		.arg("codegen")
		.arg(ecsact_src_path)
		.arg("-p")
		.arg(rust_plugin_path)
		.arg("--stdout")
		.stdout(ecsact_rs_file)
		.output()
		.unwrap();

	if !codegen_output.status.success() {
		eprint!(
			" == [Ecsact Codegen Error] ==\n{}",
			&std::str::from_utf8(&codegen_output.stderr).unwrap()
		);
		return;
	}

	Command::new("rustfmt")
		.arg("src/example_ecsact.rs")
		.spawn()
		.unwrap();
}
