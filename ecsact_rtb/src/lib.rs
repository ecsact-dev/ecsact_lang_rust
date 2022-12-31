use std::io::{BufRead, BufReader};
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

#[derive(Default)]
pub struct EcsactRuntimeBuilder {
	srcs: Vec<String>,
}

impl EcsactRuntimeBuilder {
	pub fn src(&mut self, src: &str) -> &mut EcsactRuntimeBuilder {
		self.srcs.push(src.to_string());
		self
	}

	pub fn build(&self) -> EcsactRuntime {
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
				println!("cargo:warning=INVALD JSON{}", line_str);
			}
		}

		cmd.wait().unwrap();

		runtime
	}
}
