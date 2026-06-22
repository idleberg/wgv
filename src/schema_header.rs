use regex::Regex;
use std::sync::LazyLock;

use crate::common::{ManifestTypeEnum, ManifestVer};
use crate::error::{ErrorLevel, ManifestErrorId, ValidationError};
use crate::manifest::YamlManifestInfo;
use crate::schema;

static SCHEMA_URL_PATTERN: LazyLock<Regex> =
	LazyLock::new(|| Regex::new(r"winget-manifest\.(\w+)\.([\d\.]+)\.schema\.json$").unwrap());

fn parse_schema_header_string(
	manifest_info: &YamlManifestInfo,
	error_level: ErrorLevel,
) -> Result<String, Vec<ValidationError>> {
	let header = &manifest_info.schema_header;
	if header.is_empty() {
		return Err(vec![ValidationError::message_level_with_file(
			ManifestErrorId::SchemaHeaderNotFound,
			error_level,
			&manifest_info.file_name,
		)]);
	}

	let trimmed = header.trim_start_matches('#').trim();

	// Parse "yaml-language-server: $schema=<url>" — extract the $schema value
	if let Some(rest) = trimmed.strip_prefix("yaml-language-server:") {
		let rest = rest.trim();
		if let Some(url) = rest.strip_prefix("$schema=") {
			return Ok(format!("$schema={url}"));
		}
	}

	Err(vec![
		ValidationError::message_context_value_line_level_with_file(
			ManifestErrorId::InvalidSchemaHeader,
			"",
			trimmed,
			manifest_info.schema_header_line,
			manifest_info.schema_header_column,
			error_level,
			&manifest_info.file_name,
		),
	])
}

fn validate_schema_header_type(
	header_type: &str,
	expected: ManifestTypeEnum,
	manifest_info: &YamlManifestInfo,
	error_level: ErrorLevel,
) -> Vec<ValidationError> {
	let actual = ManifestTypeEnum::parse_str(header_type);
	if actual != expected {
		let col = manifest_info
			.schema_header
			.find(header_type)
			.map(|i| i + 1)
			.unwrap_or(0);
		vec![ValidationError::message_context_value_line_level_with_file(
			ManifestErrorId::SchemaHeaderManifestTypeMismatch,
			"",
			header_type,
			manifest_info.schema_header_line,
			col,
			error_level,
			&manifest_info.file_name,
		)]
	} else {
		vec![]
	}
}

fn validate_schema_header_version(
	header_version: &str,
	expected: &ManifestVer,
	manifest_info: &YamlManifestInfo,
	error_level: ErrorLevel,
) -> Vec<ValidationError> {
	match ManifestVer::parse(header_version) {
		Some(ver) if ver == *expected => vec![],
		_ => {
			let col = manifest_info
				.schema_header
				.find(header_version)
				.map(|i| i + 1)
				.unwrap_or(0);
			vec![ValidationError::message_context_value_line_level_with_file(
				ManifestErrorId::SchemaHeaderManifestVersionMismatch,
				"",
				header_version,
				manifest_info.schema_header_line,
				col,
				error_level,
				&manifest_info.file_name,
			)]
		}
	}
}

fn is_valid_schema_header_url(
	schema_header_url: &str,
	manifest_info: &YamlManifestInfo,
	manifest_version: &ManifestVer,
) -> bool {
	if let Some(schema_json) =
		schema::load_schema_json(manifest_version, manifest_info.manifest_type)
		&& let Some(schema_id) = schema_json.get("$id").and_then(|v| v.as_str())
	{
		let expected = format!("$schema={schema_id}");
		return expected.eq_ignore_ascii_case(schema_header_url);
	}
	false
}

fn get_url_pattern_mismatch_error(
	schema_header_url: &str,
	manifest_info: &YamlManifestInfo,
	error_level: ErrorLevel,
) -> ValidationError {
	let col = manifest_info
		.schema_header
		.find(schema_header_url)
		.map(|i| i + 1)
		.unwrap_or(0);
	ValidationError::message_context_value_line_level_with_file(
		ManifestErrorId::SchemaHeaderUrlPatternMismatch,
		"",
		&manifest_info.schema_header,
		manifest_info.schema_header_line,
		col,
		error_level,
		&manifest_info.file_name,
	)
}

pub fn validate_manifests_schema_header(
	manifest_list: &[YamlManifestInfo],
	manifest_version: &ManifestVer,
	treat_error_as_warning: bool,
) -> Vec<ValidationError> {
	let mut errors = Vec::new();
	let error_level = if treat_error_as_warning {
		ErrorLevel::Warning
	} else {
		ErrorLevel::Error
	};

	for entry in manifest_list {
		if entry.manifest_type == ManifestTypeEnum::Shadow {
			continue;
		}

		if entry.schema_header.is_empty() {
			errors.push(ValidationError::message_level_with_file(
				ManifestErrorId::SchemaHeaderNotFound,
				error_level,
				&entry.file_name,
			));
			continue;
		}

		let schema_header_url = match parse_schema_header_string(entry, error_level) {
			Ok(url) => url,
			Err(mut errs) => {
				errors.append(&mut errs);
				continue;
			}
		};

		if let Some(caps) = SCHEMA_URL_PATTERN.captures(&schema_header_url) {
			let type_str = caps.get(1).unwrap().as_str();
			let version_str = caps.get(2).unwrap().as_str();

			errors.extend(validate_schema_header_type(
				type_str,
				entry.manifest_type,
				entry,
				error_level,
			));

			errors.extend(validate_schema_header_version(
				version_str,
				manifest_version,
				entry,
				error_level,
			));

			if !is_valid_schema_header_url(&schema_header_url, entry, manifest_version) {
				errors.push(get_url_pattern_mismatch_error(
					&schema_header_url,
					entry,
					error_level,
				));
			}
		} else {
			errors.push(get_url_pattern_mismatch_error(
				&schema_header_url,
				entry,
				error_level,
			));
		}
	}

	errors
}
