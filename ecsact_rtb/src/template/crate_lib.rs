// Generated by the ecsact_rtb crate. Do not edit.

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
#[allow(non_camel_case_types)]
mod bindings {{
	include!(r"{output_dir}/bindings.generated.rs");
}}

pub mod core {{
	include!(r"{output_dir}/core.rs");
}}

pub mod system_execution_context {{
	include!(r"{output_dir}/system_execution_context.rs");
}}

// Modules generated from .ecsact files

{generated_modules}

