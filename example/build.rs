use std::fs::File;
use std::path::Path;
use std::process::Command;

fn main() {
	let rust_plugin_path = "../target/debug/ecsact_rust_source_codegen.dll";
	let ecsact_src_path = "src/example.ecsact";

	println!("cargo:rerun-if-changed={}", ecsact_src_path);
	println!("cargo:rerun-if-changed={}", rust_plugin_path);

	let ecsact_rs_file = File::create("src/example.ecsact.rs").unwrap();

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
		.arg("src/example.ecsact.rs")
		.spawn()
		.unwrap();

	let runtime = ecsact_rtb::EcsactRuntime::builder()
		.src("src/example.ecsact")
		.build();

	for method in &runtime.available_methods {
		println!("cargo:rustc-cfg={method}");
	}

	println!(
		"cargo:warning=av_mthds={}",
		&runtime.available_methods.len()
	);

	let output_dir = Path::new(&runtime.output).parent().unwrap();
	let output_filename_no_ext = Path::new(&runtime.output).with_extension("");

	println!(
		"cargo:rustc-link-search=native={}",
		&output_dir.to_str().unwrap()
	);
	println!(
		"cargo:rustc-link-lib=dylib={}:escact_runtime",
		&output_filename_no_ext.to_str().unwrap()
	);
}
