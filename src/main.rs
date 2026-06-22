use std::path::PathBuf;
use std::process::ExitCode;

use clap::Parser;

use wgv::common::ManifestValidateOption;
use wgv::error::{ErrorLevel, ValidationError};
use wgv::manifest;

mod logger;
use logger::*;

#[derive(Parser)]
#[command(name = "wgv", about = "Cross-platform winget manifest validator")]
struct Cli {
	/// Path to manifest file or directory
	manifest: PathBuf,

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

	match manifest::validate_from_path(&cli.manifest, &option) {
		Ok(()) => {
			logger_success!("Manifest validation succeeded.");
			ExitCode::SUCCESS
		}
		Err(e) => {
			if e.is_warning_only() {
				logger_warn!("Manifest validation succeeded with warnings.");
				print_validation_errors(&e.errors);
				ExitCode::from(1)
			} else if let Some(syntax_err) = &e.syntax_error {
				logger_error!("{syntax_err}");
				ExitCode::from(2)
			} else {
				logger_error!("Manifest validation failed.");
				print_validation_errors(&e.errors);
				ExitCode::from(2)
			}
		}
	}
}
