use std::collections::{HashMap, HashSet};
use yaml_rust2::Yaml;

use crate::common::*;
use crate::error::*;
use crate::manifest::YamlManifestInfo;
use crate::yaml_helpers::*;

fn path_escapes_base(path: &str) -> bool {
	let normalized = path.replace('\\', "/");
	let mut depth: i32 = 0;
	for component in normalized.split('/') {
		match component {
			".." => {
				depth -= 1;
				if depth < 0 {
					return true;
				}
			}
			"" | "." => {}
			_ => depth += 1,
		}
	}
	false
}

fn is_well_formed_bcp47(tag: &str) -> bool {
	if tag.is_empty() {
		return false;
	}
	let parts: Vec<&str> = tag.split('-').collect();
	let primary = parts[0];
	if primary.len() < 2 || primary.len() > 8 || !primary.chars().all(|c| c.is_ascii_alphabetic()) {
		return false;
	}
	for part in &parts[1..] {
		if part.is_empty() || part.len() > 8 {
			return false;
		}
		if !part.chars().all(|c| c.is_ascii_alphanumeric()) {
			return false;
		}
	}
	true
}

fn is_valid_url(url_str: &str) -> bool {
	url::Url::parse(url_str).is_ok()
}

fn is_approximate_version(s: &str) -> bool {
	s.starts_with("< ") || s.starts_with("> ")
}

fn is_valid_windows_feature_name(s: &str) -> bool {
	!s.is_empty()
		&& s.chars()
			.all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
}

const ALLOWED_PORTABLE_EXTENSIONS: &[&str] = &[".exe"];
const ALLOWED_FONT_EXTENSIONS: &[&str] = &[".otf", ".ttf", ".fnt", ".ttc", ".otc"];

pub fn validate_manifest_from_yaml(manifest_list: &[YamlManifestInfo]) -> Vec<ValidationError> {
	let mut errors = Vec::new();

	let root = if manifest_list.len() == 1 {
		&manifest_list[0].root
	} else {
		let installer_doc = manifest_list
			.iter()
			.find(|m| m.manifest_type == ManifestTypeEnum::Installer);
		match installer_doc {
			Some(doc) => &doc.root,
			None => return errors,
		}
	};

	errors.extend(validate_root_fields(root));
	errors.extend(validate_installers(root));
	errors.extend(validate_all_localizations(manifest_list, root));

	errors
}

fn validate_root_fields(root: &Yaml) -> Vec<ValidationError> {
	let mut errors = Vec::new();

	if let Some(channel) = yaml_get_str(root, "Channel")
		&& !channel.is_empty()
	{
		errors.push(ValidationError::with_context_value(
			ManifestErrorId::FieldNotSupported,
			"Channel",
			&channel,
		));
	}

	if let Some(version) = yaml_get_str(root, "PackageVersion")
		&& is_approximate_version(&version)
	{
		errors.push(ValidationError::with_context_value(
			ManifestErrorId::ApproximateVersionNotAllowed,
			"PackageVersion",
			&version,
		));
	}

	errors
}

struct RootDefaults<'a> {
	installer_type_str: String,
	scope_str: String,
	switches: Option<&'a Yaml>,
	nested_type_str: String,
	nested_files: Option<&'a [Yaml]>,
}

fn validate_installers(root: &Yaml) -> Vec<ValidationError> {
	let mut errors = Vec::new();

	let Some(installers) = yaml_get_array(root, "Installers") else {
		return errors;
	};

	let defaults = RootDefaults {
		installer_type_str: yaml_get_str(root, "InstallerType").unwrap_or_default(),
		scope_str: yaml_get_str(root, "Scope").unwrap_or_default(),
		switches: yaml_get_map(root, "InstallerSwitches"),
		nested_type_str: yaml_get_str(root, "NestedInstallerType").unwrap_or_default(),
		nested_files: yaml_get_array(root, "NestedInstallerFiles"),
	};

	let mut installer_keys: Vec<(String, String, String, String, String)> = Vec::new();
	let mut duplicate_found = false;
	let mut url_to_hash: HashMap<String, String> = HashMap::new();
	let mut hash_to_url: HashMap<String, String> = HashMap::new();

	for installer in installers {
		let installer_type_str = yaml_get_str(installer, "InstallerType")
			.unwrap_or_else(|| defaults.installer_type_str.clone());
		let installer_type = InstallerType::parse_str(&installer_type_str);
		let arch_str = yaml_get_str(installer, "Architecture").unwrap_or_default();
		let arch = Architecture::parse_str(&arch_str);
		let locale = yaml_get_str(installer, "InstallerLocale").unwrap_or_default();
		let scope_str =
			yaml_get_str(installer, "Scope").unwrap_or_else(|| defaults.scope_str.clone());
		let scope = ScopeEnum::parse_str(&scope_str);

		let nested_type_for_key = if installer_type.is_archive() {
			yaml_get_str(installer, "NestedInstallerType")
				.unwrap_or_else(|| defaults.nested_type_str.clone())
				.to_lowercase()
		} else {
			String::new()
		};
		let key = (
			installer_type_str.to_lowercase(),
			nested_type_for_key,
			arch_str.to_lowercase(),
			locale.to_lowercase(),
			if scope == ScopeEnum::Unknown {
				String::new()
			} else {
				scope_str.to_lowercase()
			},
		);

		if !duplicate_found && installer_keys.contains(&key) {
			errors.push(ValidationError::new(
				ManifestErrorId::DuplicateInstallerEntry,
			));
			duplicate_found = true;
		}
		installer_keys.push(key);

		if arch == Architecture::Unknown {
			errors.push(ValidationError::with_context(
				ManifestErrorId::InvalidFieldValue,
				"Architecture",
			));
		}

		if installer_type == InstallerType::Unknown {
			errors.push(ValidationError::with_context(
				ManifestErrorId::InvalidFieldValue,
				"InstallerType",
			));
		}

		let nested_type_str_for_effective = yaml_get_str(installer, "NestedInstallerType")
			.unwrap_or_else(|| defaults.nested_type_str.clone());
		let effective_type = if installer_type.is_archive() {
			InstallerType::parse_str(&nested_type_str_for_effective)
		} else {
			installer_type
		};

		errors.extend(validate_installer_type_constraints(
			installer,
			effective_type,
			scope,
		));

		let installer_url = yaml_get_str(installer, "InstallerUrl").unwrap_or_default();
		let installer_hash = yaml_get_str(installer, "InstallerSha256").unwrap_or_default();

		errors.extend(validate_installer_url_hash(
			effective_type,
			&installer_url,
			&installer_hash,
			&mut url_to_hash,
			&mut hash_to_url,
		));

		if effective_type == InstallerType::Exe {
			errors.extend(validate_exe_switches(installer, defaults.switches));
		}

		if installer_type.is_archive() {
			errors.extend(validate_nested_installer_files(installer, &defaults));
		}

		if !installer_url.is_empty() && !is_valid_url(&installer_url) {
			errors.push(ValidationError::with_context_value(
				ManifestErrorId::InvalidFieldValue,
				"InstallerUrl",
				&installer_url,
			));
		}

		let installer_locale = yaml_get_str(installer, "InstallerLocale").unwrap_or_default();
		if !installer_locale.is_empty() && !is_well_formed_bcp47(&installer_locale) {
			errors.push(ValidationError::with_context_value(
				ManifestErrorId::InvalidBcp47Value,
				"InstallerLocale",
				&installer_locale,
			));
		}

		errors.extend(validate_markets(installer));
		errors.extend(validate_return_codes(installer));
		errors.extend(validate_windows_features(installer));
		errors.extend(validate_switch_network_addresses(
			installer,
			defaults.switches,
		));
	}

	errors
}

fn validate_installer_type_constraints(
	installer: &Yaml,
	effective_type: InstallerType,
	scope: ScopeEnum,
) -> Vec<ValidationError> {
	let mut errors = Vec::new();

	if let Some(pfn) = yaml_get_str(installer, "PackageFamilyName")
		&& !pfn.is_empty()
		&& !effective_type.uses_package_family_name()
	{
		errors.push(ValidationError::with_context_value_level(
			ManifestErrorId::InstallerTypeDoesNotSupportPackageFamilyName,
			"InstallerType",
			effective_type.as_str(),
			ErrorLevel::Warning,
		));
	}

	if let Some(pc) = yaml_get_str(installer, "ProductCode")
		&& !pc.is_empty()
		&& !effective_type.uses_product_code()
	{
		errors.push(ValidationError::with_context_value(
			ManifestErrorId::InstallerTypeDoesNotSupportProductCode,
			"InstallerType",
			effective_type.as_str(),
		));
	}

	if let Some(arp_entries) = yaml_get_array(installer, "AppsAndFeaturesEntries") {
		if !arp_entries.is_empty() && !effective_type.writes_arp_entry() {
			errors.push(ValidationError::with_context_value(
				ManifestErrorId::InstallerTypeDoesNotWriteAppsAndFeaturesEntry,
				"InstallerType",
				effective_type.as_str(),
			));
		}

		if effective_type == InstallerType::Portable && arp_entries.len() > 1 {
			errors.push(ValidationError::new(
				ManifestErrorId::ExceededAppsAndFeaturesEntryLimit,
			));
		}

		for entry in arp_entries {
			if let Some(dv) = yaml_get_str(entry, "DisplayVersion")
				&& !dv.is_empty()
				&& is_approximate_version(&dv)
			{
				errors.push(ValidationError::with_context_value(
					ManifestErrorId::ApproximateVersionNotAllowed,
					"DisplayVersion",
					&dv,
				));
			}
		}
	}

	if effective_type == InstallerType::MSStore {
		errors.push(ValidationError::with_context_value(
			ManifestErrorId::FieldValueNotSupported,
			"InstallerType",
			effective_type.as_str(),
		));
	}

	if effective_type == InstallerType::Portable {
		if let Some(cmds) = yaml_get_array(installer, "Commands")
			&& cmds.len() > 1
		{
			errors.push(ValidationError::new(ManifestErrorId::ExceededCommandsLimit));
		}
		if scope != ScopeEnum::Unknown {
			errors.push(
				ValidationError::new(ManifestErrorId::ScopeNotSupported)
					.with_level(ErrorLevel::Warning),
			);
		}
	}

	errors
}

fn validate_installer_url_hash(
	effective_type: InstallerType,
	installer_url: &str,
	installer_hash: &str,
	url_to_hash: &mut HashMap<String, String>,
	hash_to_url: &mut HashMap<String, String>,
) -> Vec<ValidationError> {
	let mut errors = Vec::new();

	if effective_type == InstallerType::MSStore {
		return errors;
	}

	if installer_url.is_empty() {
		errors.push(ValidationError::with_context(
			ManifestErrorId::RequiredFieldMissing,
			"InstallerUrl",
		));
	}
	if installer_hash.is_empty() {
		errors.push(ValidationError::with_context(
			ManifestErrorId::RequiredFieldMissing,
			"InstallerSha256",
		));
	}

	if !installer_url.is_empty() && !installer_hash.is_empty() {
		if let Some(existing_hash) = url_to_hash.get(installer_url) {
			if *existing_hash != installer_hash {
				errors.push(ValidationError::with_context_value(
					ManifestErrorId::InconsistentInstallerHash,
					"InstallerUrl",
					installer_url,
				));
			}
		} else {
			url_to_hash.insert(installer_url.to_string(), installer_hash.to_string());
		}

		if let Some(existing_url) = hash_to_url.get(installer_hash) {
			if *existing_url != installer_url {
				errors.push(ValidationError::with_context_value_level(
					ManifestErrorId::DuplicateInstallerHash,
					"InstallerSha256",
					installer_hash,
					ErrorLevel::Warning,
				));
			}
		} else {
			hash_to_url.insert(installer_hash.to_string(), installer_url.to_string());
		}
	}

	errors
}

fn validate_exe_switches(installer: &Yaml, root_switches: Option<&Yaml>) -> Vec<ValidationError> {
	let switches = yaml_get_map(installer, "InstallerSwitches").or(root_switches);
	let has_silent = switches.and_then(|s| yaml_get_str(s, "Silent")).is_some();
	let has_silent_progress = switches
		.and_then(|s| yaml_get_str(s, "SilentWithProgress"))
		.is_some();
	if !has_silent || !has_silent_progress {
		vec![
			ValidationError::new(ManifestErrorId::ExeInstallerMissingSilentSwitches)
				.with_level(ErrorLevel::Warning),
		]
	} else {
		vec![]
	}
}

fn validate_nested_installer_files(
	installer: &Yaml,
	defaults: &RootDefaults,
) -> Vec<ValidationError> {
	let mut errors = Vec::new();

	let nested_type_str = yaml_get_str(installer, "NestedInstallerType")
		.unwrap_or_else(|| defaults.nested_type_str.clone());
	let nested_type = InstallerType::parse_str(&nested_type_str);

	if nested_type == InstallerType::Unknown {
		errors.push(ValidationError::with_context(
			ManifestErrorId::RequiredFieldMissing,
			"NestedInstallerType",
		));
	}

	let nested_files = yaml_get_array(installer, "NestedInstallerFiles")
		.or(defaults.nested_files)
		.unwrap_or_default();
	if nested_files.is_empty() {
		errors.push(ValidationError::with_context(
			ManifestErrorId::RequiredFieldMissing,
			"NestedInstallerFiles",
		));
	}

	let is_portable = nested_type == InstallerType::Portable;
	let is_font = nested_type == InstallerType::Font;

	if !is_portable && !is_font && nested_files.len() != 1 && !nested_files.is_empty() {
		errors.push(ValidationError::with_context(
			ManifestErrorId::ExceededNestedInstallerFilesLimit,
			"NestedInstallerFiles",
		));
	}

	let mut alias_set = HashSet::new();
	let mut path_set = HashSet::new();

	for nf in nested_files {
		let rel_path = yaml_get_str(nf, "RelativeFilePath").unwrap_or_default();
		if rel_path.is_empty() {
			errors.push(ValidationError::with_context(
				ManifestErrorId::RequiredFieldMissing,
				"RelativeFilePath",
			));
			break;
		}

		if path_escapes_base(&rel_path) {
			errors.push(ValidationError::with_context(
				ManifestErrorId::RelativeFilePathEscapesDirectory,
				"RelativeFilePath",
			));
		}

		if !path_set.insert(rel_path.to_lowercase()) {
			errors.push(ValidationError::with_context(
				ManifestErrorId::DuplicateRelativeFilePath,
				"RelativeFilePath",
			));
		}

		let alias = yaml_get_str(nf, "PortableCommandAlias").unwrap_or_default();
		if !alias.is_empty() {
			if path_escapes_base(&alias) {
				errors.push(ValidationError::with_context(
					ManifestErrorId::PortableCommandAliasEscapesDirectory,
					"PortableCommandAlias",
				));
			}
			if !alias_set.insert(alias.to_lowercase()) {
				errors.push(ValidationError::with_context(
					ManifestErrorId::DuplicatePortableCommandAlias,
					"PortableCommandAlias",
				));
				break;
			}
		}

		if let Some(ext_pos) = rel_path.rfind('.') {
			let ext = rel_path[ext_pos..].to_lowercase();
			if is_portable && !ALLOWED_PORTABLE_EXTENSIONS.contains(&ext.as_str()) {
				errors.push(ValidationError::with_context_value(
					ManifestErrorId::InvalidPortableFiletype,
					"RelativeFilePath",
					&rel_path,
				));
			}
			if is_font && !ALLOWED_FONT_EXTENSIONS.contains(&ext.as_str()) {
				errors.push(ValidationError::with_context_value(
					ManifestErrorId::InvalidFontFiletype,
					"RelativeFilePath",
					&rel_path,
				));
			}
		}
	}

	errors
}

fn validate_markets(installer: &Yaml) -> Vec<ValidationError> {
	let has_allowed = yaml_get_array(installer, "AllowedMarkets")
		.map(|a| !a.is_empty())
		.unwrap_or(false);
	let has_excluded = yaml_get_array(installer, "ExcludedMarkets")
		.map(|a| !a.is_empty())
		.unwrap_or(false);
	if has_allowed && has_excluded {
		vec![ValidationError::new(
			ManifestErrorId::BothAllowedAndExcludedMarketsDefined,
		)]
	} else {
		vec![]
	}
}

fn validate_return_codes(installer: &Yaml) -> Vec<ValidationError> {
	let mut errors = Vec::new();
	let mut return_code_set: HashSet<i64> = HashSet::new();

	if let Some(success_codes) = yaml_get_array(installer, "InstallerSuccessCodes") {
		for code in success_codes {
			let val = match code {
				Yaml::Integer(i) => *i,
				Yaml::String(s) => s.parse::<i64>().unwrap_or(0),
				_ => continue,
			};
			return_code_set.insert(val);
		}
	}

	if let Some(expected_codes) = yaml_get_array(installer, "ExpectedReturnCodes") {
		for code_entry in expected_codes {
			let rc = match yaml_get_map(code_entry, "InstallerReturnCode") {
				Some(Yaml::Integer(i)) => Some(*i),
				Some(Yaml::String(s)) => s.parse::<i64>().ok(),
				_ => yaml_get_str(code_entry, "InstallerReturnCode")
					.and_then(|s| s.parse::<i64>().ok()),
			};
			if let Some(val) = rc
				&& !return_code_set.insert(val)
			{
				errors.push(ValidationError::new(
					ManifestErrorId::DuplicateReturnCodeEntry,
				));
				break;
			}
		}
	}

	errors
}

fn validate_windows_features(installer: &Yaml) -> Vec<ValidationError> {
	let mut errors = Vec::new();

	if let Some(deps) = yaml_get_map(installer, "Dependencies")
		&& let Some(features) = yaml_get_array(deps, "WindowsFeatures")
	{
		for feature in features {
			if let Yaml::String(name) = feature
				&& !is_valid_windows_feature_name(name)
			{
				errors.push(ValidationError::with_context(
					ManifestErrorId::InvalidWindowsFeatureName,
					name,
				));
			}
		}
	}

	errors
}

fn validate_switch_network_addresses(
	installer: &Yaml,
	root_switches: Option<&Yaml>,
) -> Vec<ValidationError> {
	let mut errors = Vec::new();

	if let Some(Yaml::Hash(switch_map)) =
		yaml_get_map(installer, "InstallerSwitches").or(root_switches)
	{
		for (_, v) in switch_map {
			if let Yaml::String(val) = v {
				if val.contains("\\\\") {
					errors.push(ValidationError::with_context(
						ManifestErrorId::ContainsNetworkAddress,
						val,
					));
				} else if val.to_lowercase().contains("http://")
					|| val.to_lowercase().contains("https://")
					|| val.to_lowercase().contains("ftp://")
				{
					errors.push(ValidationError::with_context_value_level(
						ManifestErrorId::ContainsNetworkAddress,
						val,
						"",
						ErrorLevel::Warning,
					));
				}
			}
		}
	}

	errors
}

fn validate_all_localizations(
	manifest_list: &[YamlManifestInfo],
	root: &Yaml,
) -> Vec<ValidationError> {
	let mut errors = Vec::new();

	let default_locale_root = if manifest_list.len() > 1 {
		manifest_list
			.iter()
			.find(|m| m.manifest_type == ManifestTypeEnum::DefaultLocale)
			.map(|m| &m.root)
	} else {
		Some(root)
	};

	if let Some(dl) = default_locale_root {
		errors.extend(validate_localization(dl));
	}

	if manifest_list.len() > 1 {
		for entry in manifest_list {
			if entry.manifest_type == ManifestTypeEnum::Locale {
				errors.extend(validate_localization(&entry.root));
			}
		}
	} else if let Some(localizations) = yaml_get_array(root, "Localization") {
		for loc in localizations {
			errors.extend(validate_localization(loc));
		}
	}

	errors
}

fn validate_localization(node: &Yaml) -> Vec<ValidationError> {
	let mut errors = Vec::new();

	let locale = yaml_get_str(node, "PackageLocale").unwrap_or_default();
	if !locale.is_empty() && !is_well_formed_bcp47(&locale) {
		errors.push(ValidationError::with_context_value(
			ManifestErrorId::InvalidBcp47Value,
			"PackageLocale",
			&locale,
		));
	}

	if let Some(agreements) = yaml_get_array(node, "Agreements") {
		for agreement in agreements {
			let label = yaml_get_str(agreement, "AgreementLabel").unwrap_or_default();
			let content = yaml_get_str(agreement, "Agreement").unwrap_or_default();
			let url = yaml_get_str(agreement, "AgreementUrl").unwrap_or_default();
			if label.is_empty() && content.is_empty() && url.is_empty() {
				errors.push(ValidationError::with_context(
					ManifestErrorId::InvalidFieldValue,
					"Agreements",
				));
			}
		}
	}

	errors
}
