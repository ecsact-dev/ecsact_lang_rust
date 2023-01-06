use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use std::process::{Command, Stdio};

use json::JsonValue;

#[derive(Debug, Default)]
pub struct EcsactRuntime {
	pub available_methods: Vec<String>,
	pub output: String,
}

impl EcsactRuntime {
	pub fn builder() -> EcsactRuntimeBuilder {
		EcsactRuntimeBuilder::default()
	}
}

pub struct EcsactRuntimeBuilder {
	srcs: Vec<String>,
	output_dir: String,
	lib_rs_path: String,
}

impl Default for EcsactRuntimeBuilder {
	fn default() -> Self {
		Self {
			srcs: Vec::new(),
			output_dir: "ecsact_rtb_out".to_string(),
			lib_rs_path: "src/lib.rs".to_string(),
		}
	}
}

impl EcsactRuntimeBuilder {
	pub fn src(&mut self, src: &str) -> &mut EcsactRuntimeBuilder {
		self.srcs.push(src.to_string());
		self
	}

	pub fn output_dir(&mut self, output_dir: &str) {
		self.output_dir = output_dir.to_string();
	}

	pub fn lib_rs_path(&mut self, lib_rs_path: &str) {
		self.lib_rs_path = lib_rs_path.to_string();
	}

	fn ecsact_codegen(&self) {
		// TODO(zaucy): This path is only temporary until the Ecsact SDK with
		//              the rust codgen is released.
		let rust_plugin_path = "../target/debug/ecsact_rust_source_codegen.dll";

		let codegen_output = Command::new("ecsact")
			.arg("codegen")
			.args(&self.srcs)
			.arg("-p")
			.arg(rust_plugin_path)
			.arg("-o")
			.arg(&self.output_dir)
			.output()
			.unwrap();

		if !codegen_output.status.success() {
			eprint!(
				" == [Ecsact Codegen Error] ==\n{}",
				&std::str::from_utf8(&codegen_output.stderr).unwrap()
			);
			return;
		}

		for src in &self.srcs {
			let src_rs = self.output_dir.to_owned() + "/" + src + ".rs";
			Command::new("rustfmt").arg(&src_rs).spawn().unwrap();
		}
	}

	fn build_runtime_library(&self) -> EcsactRuntime {
		let crate_name = std::env::var("CARGO_PKG_NAME").unwrap();
		let out_dir = std::env::var("OUT_DIR").unwrap();
		let out_rt =
			out_dir.to_string() + "/runtime/" + &crate_name + "_ecsact_rt.dll";
		let out_tmp = out_dir + "/tmp";
		let mut cmd = Command::new("ecsact_rtb");
		cmd.stdout(Stdio::piped());
		cmd.args(&self.srcs);
		cmd.arg("--output=".to_string() + &out_rt);
		cmd.arg("--temp_dir=".to_string() + &out_tmp);
		let mut cmd = cmd.spawn().unwrap();

		let stdout = cmd.stdout.as_mut().unwrap();
		let stdout_reader = BufReader::new(stdout);

		let mut runtime = EcsactRuntime {
			available_methods: Vec::new(),
			output: out_rt,
		};

		for line in stdout_reader.lines() {
			let line_str = &line.unwrap();
			let msg = json::parse(line_str);
			if let Ok(JsonValue::Object(msg)) = msg {
				match msg["type"].as_str().unwrap() {
					"error" => panic!("{}", msg["content"]),
					"alert" => panic!("{}", msg["content"]),
					"info" => println!("{}", msg["content"]),
					"success" => println!("{}", msg["content"]),
					"subcommand_start" => {}
					"subcommand_end" => {}
					"subcommand_stdout" => {}
					"subcommand_stderr" => {
						println!("cargo:warning={}", msg["line"])
					}
					"module_methods" => {
						let methods = &msg["methods"];
						for index in 0..methods.len() {
							let method = &methods[index];
							if method["available"] == true {
								runtime
									.available_methods
									.push(method["method_name"].to_string());
							}
						}
					}
					"warning" => println!("cargo:warning={}", msg["content"]),
					_ => println!(
						"cargo:warning='{}' message not implemented: {}",
						msg["type"],
						msg.dump(),
					),
				}
			} else {
				panic!("Invalid output from Ecsact RTB. May be a bug with the ecsact_rtb crate or the wrong version of ecsact_rtb is being used.");
			}
		}

		cmd.wait().unwrap();

		runtime
	}

	fn build_runtime_bindings(&self) {
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
			.clang_arg("--target=wasm32-unknown-unknown")
			.clang_arg("-I".to_string() + include_dir)
			.header_contents("wrapper.h", include_str!("wrapper.h"))
			.parse_callbacks(Box::new(bindgen::CargoCallbacks))
			.generate()
			.expect("Unable to generate bindings");

		bindings
			.write_to_file(
				self.output_dir.to_string() + "/bindings.generated.rs",
			)
			.expect("Couldn't write bindings!");
	}

	pub fn build(&self) -> EcsactRuntime {
		self.build_runtime_bindings();
		self.ecsact_codegen();

		let template_files = vec![
			("core.rs", include_str!("template/core.rs")),
			("dynamic.rs", include_str!("template/dynamic.rs")),
			(
				"system_execution_context.rs",
				include_str!("template/system_execution_context.rs"),
			),
		];

		let lib_rs_path_abs = std::fs::canonicalize(&self.lib_rs_path).unwrap();
		let output_dir_abs = std::fs::canonicalize(&self.output_dir).unwrap();
		let output_dir_rel = pathdiff::diff_paths(
			&output_dir_abs,
			lib_rs_path_abs.parent().unwrap(),
		)
		.unwrap();

		let mut generated_modules_str = "".to_string();

		for src in &self.srcs {
			let src_path: PathBuf = src.into();
			let src_rs_path = src_path.with_extension("ecsact.rs");
			let src_rs_file_name = src_rs_path.file_name().unwrap();
			generated_modules_str.push_str(
				format!(
					"include!(r\"{}/{}\");\n",
					&output_dir_rel.to_str().unwrap(),
					src_rs_file_name.to_str().unwrap()
				)
				.as_str(),
			);
		}

		std::fs::create_dir_all(lib_rs_path_abs.parent().unwrap()).unwrap();
		std::fs::write(
			&lib_rs_path_abs,
			format!(
				include_str!("template/crate_lib.rs"),
				output_dir = &output_dir_rel.to_str().unwrap(),
				generated_modules = &generated_modules_str,
			),
		)
		.unwrap();

		for (file_name, file_content) in template_files {
			let local_path =
				PathBuf::from(self.output_dir.to_string() + "/" + file_name);
			std::fs::create_dir_all(local_path.parent().unwrap()).unwrap();
			// TODO(zaucy): Set readonly permissions to prevent mistakes
			std::fs::write(local_path, file_content).unwrap();
		}

		let runtime = self.build_runtime_library();

		for method in &runtime.available_methods {
			println!("cargo:cfg={method}");
		}

		let target = std::env::var("TARGET").unwrap();

		if !target.contains("wasm") {
			let output_dir =
				std::path::Path::new(&runtime.output).parent().unwrap();
			let output_file_no_ext = std::path::Path::new(&runtime.output)
				.with_extension("")
				.file_name()
				.unwrap()
				.to_owned();

			println!(
				"cargo:rustc-link-search={}",
				&output_dir.to_str().unwrap()
			);
			println!(
				"cargo:rustc-link-lib={}",
				&output_file_no_ext.to_str().unwrap()
			);
		}

		runtime
	}
}
