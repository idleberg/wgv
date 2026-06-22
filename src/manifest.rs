use std::collections::HashSet;
use std::fs;
use std::path::Path;
use yaml_rust2::{Yaml, YamlLoader};

use crate::common::*;
use crate::error::*;
use crate::schema_header;
use crate::schema_validation;
use crate::validation;
use crate::yaml_helpers::*;

#[derive(Debug, Clone)]
pub struct YamlManifestInfo {
	pub root: Yaml,
	pub manifest_type: ManifestTypeEnum,
	pub file_name: String,
	pub schema_header: String,
	pub schema_header_line: usize,
	pub schema_header_column: usize,
}

fn extract_schema_header(content: &str) -> (String, usize, usize) {
	let mut first_comment = None;
	for (i, line) in content.lines().take(5).enumerate() {
		let trimmed = line.trim();
		if !trimmed.starts_with('#') {
			break;
		}
		if trimmed.contains("yaml-language-server:") {
			return (trimmed.to_string(), i + 1, 1);
		}
		if first_comment.is_none() {
			first_comment = Some((trimmed.to_string(), i + 1, 1));
		}
	}
	first_comment.unwrap_or_else(|| (String::new(), 0, 0))
}

fn validate_v1_manifest_input(entry: &YamlManifestInfo) -> Result<(), ManifestException> {
	let mut errors = Vec::new();

	if !matches!(entry.root, Yaml::Hash(_)) {
		return Err(ManifestException::syntax(format!(
			"The manifest does not contain a valid root. File: {}",
			entry.file_name
		)));
	}

	if !yaml_has_key(&entry.root, "PackageIdentifier") {
		errors.push(ValidationError::message_context_with_file(
			ManifestErrorId::RequiredFieldMissing,
			"PackageIdentifier",
			&entry.file_name,
		));
	}

	if !yaml_has_key(&entry.root, "PackageVersion") {
		errors.push(ValidationError::message_context_with_file(
			ManifestErrorId::RequiredFieldMissing,
			"PackageVersion",
			&entry.file_name,
		));
	}

	if !yaml_has_key(&entry.root, "ManifestVersion") {
		errors.push(ValidationError::message_context_with_file(
			ManifestErrorId::RequiredFieldMissing,
			"ManifestVersion",
			&entry.file_name,
		));
	}

	if !yaml_has_key(&entry.root, "ManifestType") {
		errors.push(ValidationError::message_context_with_file(
			ManifestErrorId::InconsistentMultiFileManifestFieldValue,
			"ManifestType",
			&entry.file_name,
		));
	} else {
		let manifest_type_str = yaml_get_str(&entry.root, "ManifestType").unwrap_or_default();
		let manifest_type = ManifestTypeEnum::parse_str(&manifest_type_str);

		match manifest_type {
			ManifestTypeEnum::Version if !yaml_has_key(&entry.root, "DefaultLocale") => {
				errors.push(ValidationError::message_context_with_file(
					ManifestErrorId::RequiredFieldMissing,
					"DefaultLocale",
					&entry.file_name,
				));
			}
			ManifestTypeEnum::Singleton
			| ManifestTypeEnum::Locale
			| ManifestTypeEnum::DefaultLocale
				if !yaml_has_key(&entry.root, "PackageLocale") =>
			{
				errors.push(ValidationError::message_context_with_file(
					ManifestErrorId::RequiredFieldMissing,
					"PackageLocale",
					&entry.file_name,
				));
			}
			_ => {}
		}
	}

	if !errors.is_empty() {
		return Err(ManifestException::new(errors));
	}
	Ok(())
}

fn validate_input(
	input: &mut [YamlManifestInfo],
	validate_option: &ManifestValidateOption,
) -> Result<ManifestVer, ManifestException> {
	let mut errors = Vec::new();

	let first = &input[0];
	if !matches!(first.root, Yaml::Hash(_)) {
		return Err(ManifestException::syntax(format!(
			"The manifest does not contain a valid root. File: {}",
			first.file_name
		)));
	}

	let manifest_version_str = yaml_get_str(&first.root, "ManifestVersion")
		.unwrap_or_else(|| DEFAULT_MANIFEST_VERSION.to_string());

	let manifest_version = ManifestVer::parse(&manifest_version_str).ok_or_else(|| {
		ManifestException::syntax(format!("Invalid ManifestVersion: {manifest_version_str}"))
	})?;

	if manifest_version.major > MAX_SUPPORTED_MAJOR_VERSION {
		return Err(ManifestException::syntax(format!(
			"Unsupported ManifestVersion: {manifest_version}"
		)));
	}

	let v1 = ManifestVer::parse(MANIFEST_VERSION_V1).unwrap();
	let is_multifile = input.len() > 1;

	if manifest_version < v1 {
		return Err(ManifestException::syntax(
			"Preview manifests (< 1.0.0) are not supported.".to_string(),
		));
	}

	// V1+ validation
	for entry in input.iter() {
		validate_v1_manifest_input(entry)?;
	}

	if is_multifile {
		let package_id = yaml_get_str(&input[0].root, "PackageIdentifier").unwrap_or_default();
		let package_version = yaml_get_str(&input[0].root, "PackageVersion").unwrap_or_default();

		let mut locales_set = HashSet::new();
		let mut version_found = false;
		let mut installer_found = false;
		let mut default_locale_found = false;
		let mut shadow_found = false;
		let mut default_locale_from_version = String::new();
		let mut default_locale_from_default_locale = String::new();

		for entry in input.iter_mut() {
			let local_id = yaml_get_str(&entry.root, "PackageIdentifier").unwrap_or_default();
			if local_id != package_id {
				errors.push(ValidationError::message_context_value_with_file(
					ManifestErrorId::InconsistentMultiFileManifestFieldValue,
					"PackageIdentifier",
					&local_id,
					&entry.file_name,
				));
			}

			let local_version = yaml_get_str(&entry.root, "PackageVersion").unwrap_or_default();
			if local_version != package_version {
				errors.push(ValidationError::message_context_value_with_file(
					ManifestErrorId::InconsistentMultiFileManifestFieldValue,
					"PackageVersion",
					&local_version,
					&entry.file_name,
				));
			}

			let local_manifest_version =
				yaml_get_str(&entry.root, "ManifestVersion").unwrap_or_default();
			if local_manifest_version != manifest_version_str {
				errors.push(ValidationError::message_context_value_with_file(
					ManifestErrorId::InconsistentMultiFileManifestFieldValue,
					"ManifestVersion",
					&local_manifest_version,
					&entry.file_name,
				));
			}

			let manifest_type_str = yaml_get_str(&entry.root, "ManifestType").unwrap_or_default();
			let manifest_type = ManifestTypeEnum::parse_str(&manifest_type_str);
			entry.manifest_type = manifest_type;

			match manifest_type {
				ManifestTypeEnum::Version => {
					if version_found {
						errors.push(ValidationError::message_context_value_with_file(
							ManifestErrorId::DuplicateMultiFileManifestType,
							"ManifestType",
							&manifest_type_str,
							&entry.file_name,
						));
					} else {
						version_found = true;
						default_locale_from_version =
							yaml_get_str(&entry.root, "DefaultLocale").unwrap_or_default();
					}
				}
				ManifestTypeEnum::Installer => {
					if installer_found {
						errors.push(ValidationError::message_context_value_with_file(
							ManifestErrorId::DuplicateMultiFileManifestType,
							"ManifestType",
							&manifest_type_str,
							&entry.file_name,
						));
					} else {
						installer_found = true;
					}
				}
				ManifestTypeEnum::DefaultLocale => {
					if default_locale_found {
						errors.push(ValidationError::message_context_value_with_file(
							ManifestErrorId::DuplicateMultiFileManifestType,
							"ManifestType",
							&manifest_type_str,
							&entry.file_name,
						));
					} else {
						default_locale_found = true;
						let locale = yaml_get_str(&entry.root, "PackageLocale").unwrap_or_default();
						default_locale_from_default_locale = locale.clone();
						if locales_set.contains(&locale) {
							errors.push(ValidationError::message_context_value_with_file(
								ManifestErrorId::DuplicateMultiFileManifestLocale,
								"PackageLocale",
								&locale,
								&entry.file_name,
							));
						} else {
							locales_set.insert(locale);
						}
					}
				}
				ManifestTypeEnum::Locale => {
					let locale = yaml_get_str(&entry.root, "PackageLocale").unwrap_or_default();
					if locales_set.contains(&locale) {
						errors.push(ValidationError::message_context_value_with_file(
							ManifestErrorId::DuplicateMultiFileManifestLocale,
							"PackageLocale",
							&locale,
							&entry.file_name,
						));
					} else {
						locales_set.insert(locale);
					}
				}
				ManifestTypeEnum::Shadow => {
					if !validate_option.allow_shadow_manifest {
						errors.push(ValidationError::message_context_value_with_file(
							ManifestErrorId::ShadowManifestNotAllowed,
							"ManifestType",
							&manifest_type_str,
							&entry.file_name,
						));
					} else if shadow_found {
						errors.push(ValidationError::message_context_value_with_file(
							ManifestErrorId::DuplicateMultiFileManifestType,
							"ManifestType",
							&manifest_type_str,
							&entry.file_name,
						));
					} else {
						shadow_found = true;
					}
				}
				_ => {
					errors.push(ValidationError::message_context_value_with_file(
						ManifestErrorId::UnsupportedMultiFileManifestType,
						"ManifestType",
						&manifest_type_str,
						&entry.file_name,
					));
				}
			}
		}

		if version_found
			&& default_locale_found
			&& default_locale_from_default_locale != default_locale_from_version
		{
			errors.push(ValidationError::new(
				ManifestErrorId::InconsistentMultiFileManifestDefaultLocale,
			));
		}

		if !(validate_option.schema_validation_only
			|| version_found && installer_found && default_locale_found)
		{
			errors.push(ValidationError::new(
				ManifestErrorId::IncompleteMultiFileManifest,
			));
		}
	} else {
		let manifest_type_str = yaml_get_str(&input[0].root, "ManifestType").unwrap_or_default();
		let manifest_type = ManifestTypeEnum::parse_str(&manifest_type_str);
		input[0].manifest_type = manifest_type;

		if validate_option.full_validation && manifest_type == ManifestTypeEnum::Merged {
			errors.push(ValidationError::message_context_value_with_file(
				ManifestErrorId::FieldValueNotSupported,
				"ManifestType",
				&manifest_type_str,
				&input[0].file_name,
			));
		}

		if !validate_option.schema_validation_only
			&& manifest_type != ManifestTypeEnum::Merged
			&& manifest_type != ManifestTypeEnum::Singleton
		{
			errors.push(ValidationError::message_with_file(
				ManifestErrorId::IncompleteMultiFileManifest,
				&input[0].file_name,
			));
		}
	}

	if !errors.is_empty() {
		return Err(ManifestException::new(errors));
	}

	Ok(manifest_version)
}

fn load_yaml_file(path: &Path) -> Result<(Yaml, String, usize, usize), ManifestException> {
	let raw = fs::read_to_string(path).map_err(|e| {
		ManifestException::syntax(format!("Failed to read file {}: {e}", path.display()))
	})?;
	let content = raw.strip_prefix('\u{FEFF}').unwrap_or(&raw);

	let (header, header_line, header_col) = extract_schema_header(content);

	let docs = YamlLoader::load_from_str(content)
		.map_err(|e| ManifestException::syntax(format!("{e}")))?;

	let root = docs.into_iter().next().unwrap_or(Yaml::Null);
	Ok((root, header, header_line, header_col))
}

pub fn validate_from_path(
	input_path: &Path,
	validate_option: &ManifestValidateOption,
) -> Result<(), ManifestException> {
	let mut doc_list = Vec::new();

	if input_path.is_dir() {
		let mut entries: Vec<_> = fs::read_dir(input_path)
			.map_err(|e| {
				ManifestException::syntax(format!(
					"Failed to read directory {}: {e}",
					input_path.display()
				))
			})?
			.filter_map(|e| e.ok())
			.collect();
		entries.sort_by_key(|e| e.file_name());

		for entry in entries {
			let path = entry.path();
			if path.is_dir() {
				return Err(ManifestException::syntax(
					"Subdirectory not supported in manifest path".to_string(),
				));
			}
			let (root, header, header_line, header_col) = load_yaml_file(&path)?;
			doc_list.push(YamlManifestInfo {
				root,
				manifest_type: ManifestTypeEnum::Singleton,
				file_name: path
					.file_name()
					.map(|f| f.to_string_lossy().to_string())
					.unwrap_or_default(),
				schema_header: header,
				schema_header_line: header_line,
				schema_header_column: header_col,
			});
		}
	} else {
		let (root, header, header_line, header_col) = load_yaml_file(input_path)?;
		doc_list.push(YamlManifestInfo {
			root,
			manifest_type: ManifestTypeEnum::Singleton,
			file_name: input_path
				.file_name()
				.map(|f| f.to_string_lossy().to_string())
				.unwrap_or_default(),
			schema_header: header,
			schema_header_line: header_line,
			schema_header_column: header_col,
		});
	}

	if doc_list.is_empty() {
		return Err(ManifestException::syntax(
			"No manifest file found".to_string(),
		));
	}

	parse_manifest(&mut doc_list, validate_option)
}

fn parse_manifest(
	input: &mut [YamlManifestInfo],
	validate_option: &ManifestValidateOption,
) -> Result<(), ManifestException> {
	let manifest_version = validate_input(input, validate_option)?;

	let mut result_errors = Vec::new();

	if validate_option.full_validation || validate_option.schema_validation_only {
		let schema_errors = schema_validation::validate_against_schema(input, &manifest_version);
		result_errors.extend(schema_errors);
	}

	if validate_option.schema_validation_only {
		if !result_errors.is_empty() {
			return Err(ManifestException::new(result_errors));
		}
		return Ok(());
	}

	// Semantic validation
	if validate_option.full_validation {
		let semantic_errors = validation::validate_manifest_from_yaml(input);
		result_errors.extend(semantic_errors);

		// Schema header validation for v1.7+
		let v1_7 = ManifestVer::parse(MANIFEST_VERSION_V1_7).unwrap();
		if manifest_version >= v1_7 {
			let header_errors = schema_header::validate_manifests_schema_header(
				input,
				&manifest_version,
				validate_option.schema_header_validation_as_warning,
			);
			result_errors.extend(header_errors);
		}
	}

	if !result_errors.is_empty() {
		let ex = ManifestException::new(result_errors);
		if validate_option.throw_on_warning || !ex.is_warning_only() {
			return Err(ex);
		}
	}

	Ok(())
}
