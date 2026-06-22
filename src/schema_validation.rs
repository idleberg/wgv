use yaml_rust2::Yaml;

use crate::common::{ManifestTypeEnum, ManifestVer};
use crate::error::{ManifestErrorId, ValidationError};
use crate::manifest::YamlManifestInfo;
use crate::schema;

#[derive(Debug, Clone, Copy)]
enum YamlScalarType {
	String,
	Int,
	Bool,
}

fn get_scalar_type(key: &str) -> YamlScalarType {
	match key {
		"InstallerSuccessCodes" | "InstallerReturnCode" => YamlScalarType::Int,
		"InstallerAbortsTerminal"
		| "InstallLocationRequired"
		| "RequireExplicitUpgrade"
		| "DisplayInstallWarnings"
		| "DownloadCommandProhibited"
		| "ArchiveBinariesDependOnPath" => YamlScalarType::Bool,
		_ => YamlScalarType::String,
	}
}

fn yaml_to_json(node: &Yaml, scalar_type: YamlScalarType) -> serde_json::Value {
	match node {
		Yaml::Null => serde_json::Value::Null,
		Yaml::Hash(map) => {
			let mut obj = serde_json::Map::new();
			for (k, v) in map {
				if let Yaml::String(key) = k {
					let child_type = get_scalar_type(key);
					obj.insert(key.clone(), yaml_to_json(v, child_type));
				}
			}
			serde_json::Value::Object(obj)
		}
		Yaml::Array(arr) => {
			let items: Vec<serde_json::Value> =
				arr.iter().map(|v| yaml_to_json(v, scalar_type)).collect();
			serde_json::Value::Array(items)
		}
		Yaml::String(s) => match scalar_type {
			YamlScalarType::Int => s
				.parse::<i64>()
				.map(serde_json::Value::from)
				.unwrap_or_else(|_| serde_json::Value::String(s.clone())),
			YamlScalarType::Bool => match s.to_lowercase().as_str() {
				"true" => serde_json::Value::Bool(true),
				"false" => serde_json::Value::Bool(false),
				_ => serde_json::Value::String(s.clone()),
			},
			YamlScalarType::String => serde_json::Value::String(s.clone()),
		},
		Yaml::Integer(i) => match scalar_type {
			YamlScalarType::Int => serde_json::Value::from(*i),
			_ => serde_json::Value::String(i.to_string()),
		},
		Yaml::Real(s) => match scalar_type {
			YamlScalarType::Int => s
				.parse::<f64>()
				.map(serde_json::Value::from)
				.unwrap_or_else(|_| serde_json::Value::String(s.clone())),
			_ => serde_json::Value::String(s.clone()),
		},
		Yaml::Boolean(b) => serde_json::Value::Bool(*b),
		_ => serde_json::Value::Null,
	}
}

pub fn validate_against_schema(
	manifest_list: &[YamlManifestInfo],
	manifest_version: &ManifestVer,
) -> Vec<ValidationError> {
	let mut errors = Vec::new();

	for entry in manifest_list {
		if entry.manifest_type == ManifestTypeEnum::Shadow {
			continue;
		}

		let Some(schema_json) = schema::load_schema_json(manifest_version, entry.manifest_type)
		else {
			errors.push(ValidationError::message_context_with_file(
				ManifestErrorId::SchemaError,
				format!("No schema found for type {:?}", entry.manifest_type),
				&entry.file_name,
			));
			continue;
		};

		let manifest_json = yaml_to_json(&entry.root, YamlScalarType::String);

		let validator = match jsonschema::validator_for(&schema_json) {
			Ok(v) => v,
			Err(e) => {
				errors.push(ValidationError::message_context_with_file(
					ManifestErrorId::SchemaError,
					format!("Failed to compile schema: {e}"),
					&entry.file_name,
				));
				continue;
			}
		};

		if !validator.is_valid(&manifest_json) {
			let validation_errors = validator.iter_errors(&manifest_json);
			let error_messages: Vec<String> = validation_errors
				.map(|e| {
					let path = e.instance_path.to_string();
					if path.is_empty() {
						e.to_string()
					} else {
						format!("{path}: {e}")
					}
				})
				.collect();

			let combined = error_messages.join("; ");
			errors.push(ValidationError::message_context_with_file(
				ManifestErrorId::SchemaError,
				combined,
				&entry.file_name,
			));
		}
	}

	errors
}
