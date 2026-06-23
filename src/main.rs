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

	/// Set log level [possible values: error, warn, info]
	#[arg(long, default_value = "info", conflicts_with = "quiet")]
	log_level: String,

	/// Only show errors (shorthand for --log-level error)
	#[arg(short, long)]
	quiet: bool,
}

fn print_validation_errors(errors: &[ValidationError]) {
	let min_level = if logger::log_level() == 0 {
		ErrorLevel::Error
	} else {
		ErrorLevel::Warning
	};

	for error in errors {
		if error.error_level == ErrorLevel::Warning && min_level == ErrorLevel::Error {
			continue;
		}
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

fn validate_multi(path: &Path, option: &ManifestValidateOption) -> u8 {
	logger_info!("Validating {} ...", path.display());
	let code = validate_one(path, option);
	if logger::log_level() >= 2 {
		eprintln!();
	}
	code
}

fn main() -> ExitCode {
	let cli = Cli::parse();

	if cli.quiet {
		logger::set_log_level("error");
	} else {
		logger::set_log_level(&cli.log_level);
	}

	let option = ManifestValidateOption {
		full_validation: true,
		schema_header_validation_as_warning: true,
		throw_on_warning: logger::log_level() >= 1,
		..Default::default()
	};

	let mut iter = match discovery::discover_manifest_paths(&cli.manifests) {
		Ok(it) => it,
		Err(e) => {
			logger_error!("{e}");
			return ExitCode::from(2);
		}
	};

	let Some(first) = iter.next() else {
		logger_error!("No manifests found");
		return ExitCode::from(2);
	};

	let Some(second) = iter.next() else {
		return ExitCode::from(validate_one(&first, &option));
	};

	let mut worst: u8 = 0;
	let mut passed: usize = 0;
	let mut failed: usize = 0;
	let mut warned: usize = 0;

	let mut tally = |code: u8| {
		match code {
			0 => passed += 1,
			1 => warned += 1,
			_ => failed += 1,
		}
		if code > worst {
			worst = code;
		}
	};

	tally(validate_multi(&first, &option));
	tally(validate_multi(&second, &option));

	for path in iter {
		tally(validate_multi(&path, &option));
	}

	let total = passed + warned + failed;
	logger_info!("Results: {passed} passed, {failed} failed, {warned} warnings ({total} total)");

	ExitCode::from(worst)
}
