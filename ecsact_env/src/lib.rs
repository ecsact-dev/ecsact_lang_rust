use serde::Deserialize;
use std::{path::PathBuf, process::Command};

const ECSACT_SDK_VERSION_ERROR: &str = r#"
Failed to get Ecsact SDK version. Please make sure the Ecsact SDK is installed and ecsact is available on your PATH.

SEE: https://ecsact.dev/start
"#;

const ECSACT_SDK_CONFIG_ERROR: &str = r#"
Failed to get Ecsact SDK config. Please make sure the Ecsact SDK is installed and ecsact is available on your PATH.

SEE: https://ecsact.dev/start
"#;

#[derive(Deserialize)]
pub struct EcsactSdkConfig {
	pub builtin_plugins: Vec<String>,
	pub include_dir: PathBuf,
	pub install_dir: PathBuf,
	pub plugin_dir: PathBuf,
}

pub fn sdk_config() -> EcsactSdkConfig {
	serde_json::from_str(&String::from_utf8_lossy(
		&Command::new("ecsact")
			.arg("config")
			.output()
			.expect(ECSACT_SDK_CONFIG_ERROR)
			.stdout,
	))
	.expect(ECSACT_SDK_CONFIG_ERROR)
}

#[cfg(not(feature = "cargo"))]
pub fn sdk_include_dir() -> PathBuf {
	let runfiles = runfiles::Runfiles::create().unwrap();
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

#[cfg(feature = "cargo")]
pub fn sdk_include_dir() -> PathBuf {
	String::from_utf8_lossy(
		&Command::new("ecsact")
			.arg("config")
			.arg("include_dir")
			.output()
			.expect(ECSACT_SDK_CONFIG_ERROR)
			.stdout,
	)
	.to_string()
	.into()
}

pub fn sdk_version() -> String {
	String::from_utf8_lossy(
		&Command::new("ecsact")
			.arg("--version")
			.output()
			.expect(ECSACT_SDK_VERSION_ERROR)
			.stdout,
	)
	.to_string()
}

#[test]
fn test_sdk_version() {
	assert!(!sdk_version().is_empty());
}

#[test]
fn test_sdk_include_dir() {
	assert!(sdk_include_dir().exists());
}

#[test]
fn test_sdk_config() {
	let test_config = sdk_config();
	assert!(!test_config.builtin_plugins.is_empty());
	for builtin_plugin in test_config.builtin_plugins {
		assert!(!builtin_plugin.is_empty());
	}

	assert!(test_config.include_dir.exists());
	assert!(test_config.install_dir.exists());
	assert!(test_config.plugin_dir.exists());
}
