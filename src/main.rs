use std::path::Path;
use std::process::ExitCode;

use clap::Parser;

use wgv::common::ManifestValidateOption;
use wgv::discovery;
use wgv::error::{ErrorLevel, ValidationError};
use wgv::manifest;

mod logger;
use logger::*;

#[derive(Parser)]
#[command(
	name = "wgv",
	about = "Cross-platform winget manifest validator",
	version
)]
struct Cli {
	/// Paths or glob patterns to manifest files or directories
	#[arg(required = true)]
	manifests: Vec<String>,

	/// Ignore warnings during validation
	#[arg(long)]
	ignore_warnings: bool,

	/// Suppress informational output (errors and warnings are always shown)
	#[arg(short = 'S', long)]
	silent: bool,
}

fn print_validation_errors(errors: &[ValidationError]) {
	for error in errors {
		let prefix = match error.error_level {
			ErrorLevel::Error => "\x1b[31mManifest Error:\x1b[0m",
			ErrorLevel::Warning => "\x1b[33mManifest Warning:\x1b[0m",
		};
		let mut line = format!("{prefix} {}", error.message.message());
		if !error.context.is_empty() {
			line.push_str(&format!(" [{}]", error.context));
		}
		if !error.value.is_empty() {
			line.push_str(&format!(" Value: {}", dim(&error.value)));
		}
		if error.line > 0 && error.column > 0 {
			line.push_str(&format!(
				" {}",
				dim(&format_args!(
					"Line: {}, Column: {}",
					error.line, error.column
				))
			));
		}
		if !error.file_name.is_empty() {
			line.push_str(&format!(" File: {}", dim(&error.file_name)));
		}
		eprintln!("  {line}");
	}
}

fn validate_one(path: &Path, option: &ManifestValidateOption) -> u8 {
	match manifest::validate_from_path(path, option) {
		Ok(()) => {
			logger_success!("Manifest validation succeeded.");
			0
		}
		Err(e) => {
			if e.is_warning_only() {
				logger_warn!("Manifest validation succeeded with warnings.");
				print_validation_errors(&e.errors);
				1
			} else if let Some(syntax_err) = &e.syntax_error {
				logger_error!("{syntax_err}");
				2
			} else {
				logger_error!("Manifest validation failed.");
				print_validation_errors(&e.errors);
				2
			}
		}
	}
}

fn main() -> ExitCode {
	let cli = Cli::parse();

	if cli.silent {
		logger::SILENT.store(true, std::sync::atomic::Ordering::Relaxed);
	}

	let option = ManifestValidateOption {
		full_validation: true,
		schema_header_validation_as_warning: true,
		throw_on_warning: !cli.ignore_warnings,
		..Default::default()
	};

	logger_info!("Discovering manifests...");
	let paths = match discovery::resolve_manifest_paths(&cli.manifests) {
		Ok(p) => p,
		Err(e) => {
			logger_error!("{e}");
			return ExitCode::from(2);
		}
	};
	logger_info!("Found {} manifest(s).", paths.len());

	if paths.len() == 1 {
		return ExitCode::from(validate_one(&paths[0], &option));
	}

	let mut worst: u8 = 0;
	let mut passed: usize = 0;
	let mut failed: usize = 0;
	let mut warned: usize = 0;

	for path in &paths {
		logger_info!("Validating {} ...", path.display());
		let code = validate_one(path, &option);
		match code {
			0 => passed += 1,
			1 => warned += 1,
			_ => failed += 1,
		}
		if code > worst {
			worst = code;
		}
		if !cli.silent {
			eprintln!();
		}
	}

	let total = passed + warned + failed;
	logger_info!("Results: {passed} passed, {failed} failed, {warned} warnings ({total} total)");

	ExitCode::from(worst)
}
