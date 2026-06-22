use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ManifestErrorId {
	InvalidRootNode,
	FieldUnknown,
	FieldIsNotPascalCase,
	FieldDuplicate,
	RequiredFieldEmpty,
	RequiredFieldMissing,
	InvalidFieldValue,
	ExeInstallerMissingSilentSwitches,
	FieldNotSupported,
	FieldValueNotSupported,
	DuplicateInstallerEntry,
	DuplicateInstallerHash,
	InstallerTypeDoesNotSupportPackageFamilyName,
	InstallerTypeDoesNotSupportProductCode,
	InstallerTypeDoesNotWriteAppsAndFeaturesEntry,
	IncompleteMultiFileManifest,
	InconsistentMultiFileManifestFieldValue,
	DuplicatePortableCommandAlias,
	DuplicateRelativeFilePath,
	DuplicateMultiFileManifestType,
	DuplicateMultiFileManifestLocale,
	UnsupportedMultiFileManifestType,
	InconsistentInstallerHash,
	InconsistentMultiFileManifestDefaultLocale,
	FieldFailedToProcess,
	InvalidBcp47Value,
	BothAllowedAndExcludedMarketsDefined,
	DuplicateReturnCodeEntry,
	FieldRequireVerifiedPublisher,
	SingleManifestPackageHasDependencies,
	MultiManifestPackageHasDependencies,
	MissingManifestDependenciesNode,
	NoSuitableMinVersionDependency,
	FoundDependencyLoop,
	ExceededAppsAndFeaturesEntryLimit,
	ExceededCommandsLimit,
	ScopeNotSupported,
	InstallerMsixInconsistencies,
	OptionalFieldMissing,
	InstallerFailedToProcess,
	NoSupportedPlatforms,
	ApproximateVersionNotAllowed,
	ArpVersionOverlapWithIndex,
	ArpVersionValidationInternalError,
	ExceededNestedInstallerFilesLimit,
	PortableCommandAliasEscapesDirectory,
	RelativeFilePathEscapesDirectory,
	ArpValidationError,
	SchemaError,
	MsixSignatureHashFailed,
	ShadowManifestNotAllowed,
	SchemaHeaderNotFound,
	InvalidSchemaHeader,
	SchemaHeaderManifestTypeMismatch,
	SchemaHeaderManifestVersionMismatch,
	SchemaHeaderUrlPatternMismatch,
	InvalidPortableFiletype,
	InvalidFontFiletype,
	InvalidWindowsFeatureName,
	BlockedMsiProperty,
	InvalidMsiSwitches,
	ContainsNetworkAddress,
}

impl ManifestErrorId {
	pub fn message(&self) -> &'static str {
		match self {
			Self::InvalidRootNode => "Encountered unexpected root node.",
			Self::FieldUnknown => "Unknown field.",
			Self::FieldIsNotPascalCase => "All field names should be PascalCased.",
			Self::FieldDuplicate => "Duplicate field found in the manifest.",
			Self::RequiredFieldEmpty => "Required field with empty value.",
			Self::RequiredFieldMissing => "Required field missing.",
			Self::InvalidFieldValue => "Invalid field value.",
			Self::ExeInstallerMissingSilentSwitches => {
				"Silent and SilentWithProgress switches are not specified for InstallerType exe. Please make sure the installer can run unattended."
			}
			Self::FieldNotSupported => "Field is not supported.",
			Self::FieldValueNotSupported => "Field value is not supported.",
			Self::DuplicateInstallerEntry => "Duplicate installer entry found.",
			Self::DuplicateInstallerHash => {
				"Multiple Installer URLs found with the same InstallerSha256. Please ensure the accuracy of the URLs."
			}
			Self::InstallerTypeDoesNotSupportPackageFamilyName => {
				"The specified installer type does not support PackageFamilyName."
			}
			Self::InstallerTypeDoesNotSupportProductCode => {
				"The specified installer type does not support ProductCode."
			}
			Self::InstallerTypeDoesNotWriteAppsAndFeaturesEntry => {
				"The specified installer type does not write to Apps and Features entry."
			}
			Self::IncompleteMultiFileManifest => {
				"The multi file manifest is incomplete. A multi file manifest must contain at least version, installer and defaultLocale manifest."
			}
			Self::InconsistentMultiFileManifestFieldValue => {
				"The multi file manifest has inconsistent field values."
			}
			Self::DuplicatePortableCommandAlias => "Duplicate portable command alias found.",
			Self::DuplicateRelativeFilePath => "Duplicate relative file path found.",
			Self::DuplicateMultiFileManifestType => {
				"The multi file manifest should contain only one file with the particular ManifestType."
			}
			Self::DuplicateMultiFileManifestLocale => {
				"The multi file manifest contains duplicate PackageLocale."
			}
			Self::UnsupportedMultiFileManifestType => {
				"The multi file manifest should not contain file with the particular ManifestType."
			}
			Self::InconsistentInstallerHash => {
				"The values of InstallerSha256 do not match for all instances of the same InstallerUrl."
			}
			Self::InconsistentMultiFileManifestDefaultLocale => {
				"DefaultLocale value in version manifest does not match PackageLocale value in defaultLocale manifest."
			}
			Self::FieldFailedToProcess => "Failed to process field.",
			Self::InvalidBcp47Value => "The locale value is not a well formed bcp47 language tag.",
			Self::BothAllowedAndExcludedMarketsDefined => {
				"Both AllowedMarkets and ExcludedMarkets defined."
			}
			Self::DuplicateReturnCodeEntry => "Duplicate installer return code found.",
			Self::FieldRequireVerifiedPublisher => "Field usage requires verified publishers.",
			Self::SingleManifestPackageHasDependencies => {
				"Package has a single manifest and is a dependency of other manifests."
			}
			Self::MultiManifestPackageHasDependencies => {
				"Deleting the manifest will be break the following dependencies."
			}
			Self::MissingManifestDependenciesNode => "Dependency not found: ",
			Self::NoSuitableMinVersionDependency => "No Suitable Minimum Version: ",
			Self::FoundDependencyLoop => "Loop found.",
			Self::ExceededAppsAndFeaturesEntryLimit => {
				"Only zero or one entry for Apps and Features may be specified for InstallerType portable."
			}
			Self::ExceededCommandsLimit => {
				"Only zero or one value for Commands may be specified for InstallerType portable."
			}
			Self::ScopeNotSupported => "Scope is not supported for InstallerType portable.",
			Self::InstallerMsixInconsistencies => "Inconsistent value in the manifest.",
			Self::OptionalFieldMissing => "Optional field missing.",
			Self::InstallerFailedToProcess => "Failed to process installer.",
			Self::NoSupportedPlatforms => "No supported platforms.",
			Self::ApproximateVersionNotAllowed => "Approximate version not allowed.",
			Self::ArpVersionOverlapWithIndex => {
				"DisplayVersion declared in the manifest has overlap with existing DisplayVersion range in the index. Existing DisplayVersion range in index: "
			}
			Self::ArpVersionValidationInternalError => {
				"Internal error while validating DisplayVersion against index."
			}
			Self::ExceededNestedInstallerFilesLimit => {
				"Only one entry for NestedInstallerFiles can be specified for non-portable InstallerTypes."
			}
			Self::PortableCommandAliasEscapesDirectory => {
				"Portable command alias must not point to a location outside of base directory."
			}
			Self::RelativeFilePathEscapesDirectory => {
				"Relative file path must not point to a location outside of archive directory."
			}
			Self::ArpValidationError => "Arp Validation Error.",
			Self::SchemaError => "Schema Error.",
			Self::MsixSignatureHashFailed => {
				"Failed to calculate MSIX signature hash. Please verify that the input file is a valid, signed MSIX."
			}
			Self::ShadowManifestNotAllowed => "Shadow manifest is not allowed.",
			Self::SchemaHeaderNotFound => "Schema header not found.",
			Self::InvalidSchemaHeader => {
				"The schema header is invalid. Please verify that the schema header is present and formatted correctly."
			}
			Self::SchemaHeaderManifestTypeMismatch => {
				"The manifest type in the schema header does not match the ManifestType property value in the manifest."
			}
			Self::SchemaHeaderManifestVersionMismatch => {
				"The manifest version in the schema header does not match the ManifestVersion property value in the manifest."
			}
			Self::SchemaHeaderUrlPatternMismatch => {
				"The schema header URL does not match the expected pattern."
			}
			Self::InvalidPortableFiletype => "The file type of the referenced file is not allowed.",
			Self::InvalidFontFiletype => {
				"The file type of the referenced file is not a supported font file type."
			}
			Self::InvalidWindowsFeatureName => {
				"The provided value is not a valid Windows feature name."
			}
			Self::BlockedMsiProperty => "Contains a blocked MSI property.",
			Self::InvalidMsiSwitches => "Contains invalid MSI switches.",
			Self::ContainsNetworkAddress => "Installer switch contains network address.",
		}
	}

	pub fn as_str(&self) -> &'static str {
		match self {
			Self::InvalidRootNode => "InvalidRootNode",
			Self::FieldUnknown => "FieldUnknown",
			Self::FieldIsNotPascalCase => "FieldIsNotPascalCase",
			Self::FieldDuplicate => "FieldDuplicate",
			Self::RequiredFieldEmpty => "RequiredFieldEmpty",
			Self::RequiredFieldMissing => "RequiredFieldMissing",
			Self::InvalidFieldValue => "InvalidFieldValue",
			Self::ExeInstallerMissingSilentSwitches => "ExeInstallerMissingSilentSwitches",
			Self::FieldNotSupported => "FieldNotSupported",
			Self::FieldValueNotSupported => "FieldValueNotSupported",
			Self::DuplicateInstallerEntry => "DuplicateInstallerEntry",
			Self::DuplicateInstallerHash => "DuplicateInstallerHash",
			Self::InstallerTypeDoesNotSupportPackageFamilyName => {
				"InstallerTypeDoesNotSupportPackageFamilyName"
			}
			Self::InstallerTypeDoesNotSupportProductCode => {
				"InstallerTypeDoesNotSupportProductCode"
			}
			Self::InstallerTypeDoesNotWriteAppsAndFeaturesEntry => {
				"InstallerTypeDoesNotWriteAppsAndFeaturesEntry"
			}
			Self::IncompleteMultiFileManifest => "IncompleteMultiFileManifest",
			Self::InconsistentMultiFileManifestFieldValue => {
				"InconsistentMultiFileManifestFieldValue"
			}
			Self::DuplicatePortableCommandAlias => "DuplicatePortableCommandAlias",
			Self::DuplicateRelativeFilePath => "DuplicateRelativeFilePath",
			Self::DuplicateMultiFileManifestType => "DuplicateMultiFileManifestType",
			Self::DuplicateMultiFileManifestLocale => "DuplicateMultiFileManifestLocale",
			Self::UnsupportedMultiFileManifestType => "UnsupportedMultiFileManifestType",
			Self::InconsistentInstallerHash => "InconsistentInstallerHash",
			Self::InconsistentMultiFileManifestDefaultLocale => {
				"InconsistentMultiFileManifestDefaultLocale"
			}
			Self::FieldFailedToProcess => "FieldFailedToProcess",
			Self::InvalidBcp47Value => "InvalidBcp47Value",
			Self::BothAllowedAndExcludedMarketsDefined => "BothAllowedAndExcludedMarketsDefined",
			Self::DuplicateReturnCodeEntry => "DuplicateReturnCodeEntry",
			Self::FieldRequireVerifiedPublisher => "FieldRequireVerifiedPublisher",
			Self::SingleManifestPackageHasDependencies => "SingleManifestPackageHasDependencies",
			Self::MultiManifestPackageHasDependencies => "MultiManifestPackageHasDependencies",
			Self::MissingManifestDependenciesNode => "MissingManifestDependenciesNode",
			Self::NoSuitableMinVersionDependency => "NoSuitableMinVersionDependency",
			Self::FoundDependencyLoop => "FoundDependencyLoop",
			Self::ExceededAppsAndFeaturesEntryLimit => "ExceededAppsAndFeaturesEntryLimit",
			Self::ExceededCommandsLimit => "ExceededCommandsLimit",
			Self::ScopeNotSupported => "ScopeNotSupported",
			Self::InstallerMsixInconsistencies => "InstallerMsixInconsistencies",
			Self::OptionalFieldMissing => "OptionalFieldMissing",
			Self::InstallerFailedToProcess => "InstallerFailedToProcess",
			Self::NoSupportedPlatforms => "NoSupportedPlatforms",
			Self::ApproximateVersionNotAllowed => "ApproximateVersionNotAllowed",
			Self::ArpVersionOverlapWithIndex => "ArpVersionOverlapWithIndex",
			Self::ArpVersionValidationInternalError => "ArpVersionValidationInternalError",
			Self::ExceededNestedInstallerFilesLimit => "ExceededNestedInstallerFilesLimit",
			Self::PortableCommandAliasEscapesDirectory => "PortableCommandAliasEscapesDirectory",
			Self::RelativeFilePathEscapesDirectory => "RelativeFilePathEscapesDirectory",
			Self::ArpValidationError => "ArpValidationError",
			Self::SchemaError => "SchemaError",
			Self::MsixSignatureHashFailed => "MsixSignatureHashFailed",
			Self::ShadowManifestNotAllowed => "ShadowManifestNotAllowed",
			Self::SchemaHeaderNotFound => "SchemaHeaderNotFound",
			Self::InvalidSchemaHeader => "InvalidSchemaHeader",
			Self::SchemaHeaderManifestTypeMismatch => "SchemaHeaderManifestTypeMismatch",
			Self::SchemaHeaderManifestVersionMismatch => "SchemaHeaderManifestVersionMismatch",
			Self::SchemaHeaderUrlPatternMismatch => "SchemaHeaderUrlPatternMismatch",
			Self::InvalidPortableFiletype => "InvalidPortableFiletype",
			Self::InvalidFontFiletype => "InvalidFontFiletype",
			Self::InvalidWindowsFeatureName => "InvalidWindowsFeatureName",
			Self::BlockedMsiProperty => "BlockedMsiProperty",
			Self::InvalidMsiSwitches => "InvalidMsiSwitches",
			Self::ContainsNetworkAddress => "ContainsNetworkAddress",
		}
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorLevel {
	Warning,
	Error,
}

#[derive(Debug, Clone)]
pub struct ValidationError {
	pub message: ManifestErrorId,
	pub context: String,
	pub value: String,
	pub line: usize,
	pub column: usize,
	pub error_level: ErrorLevel,
	pub file_name: String,
}

impl ValidationError {
	pub fn new(message: ManifestErrorId) -> Self {
		Self {
			message,
			context: String::new(),
			value: String::new(),
			line: 0,
			column: 0,
			error_level: ErrorLevel::Error,
			file_name: String::new(),
		}
	}

	pub fn with_context(message: ManifestErrorId, context: impl Into<String>) -> Self {
		Self {
			message,
			context: context.into(),
			value: String::new(),
			line: 0,
			column: 0,
			error_level: ErrorLevel::Error,
			file_name: String::new(),
		}
	}

	pub fn with_context_value(
		message: ManifestErrorId,
		context: impl Into<String>,
		value: impl Into<String>,
	) -> Self {
		Self {
			message,
			context: context.into(),
			value: value.into(),
			line: 0,
			column: 0,
			error_level: ErrorLevel::Error,
			file_name: String::new(),
		}
	}

	pub fn with_context_value_level(
		message: ManifestErrorId,
		context: impl Into<String>,
		value: impl Into<String>,
		level: ErrorLevel,
	) -> Self {
		Self {
			message,
			context: context.into(),
			value: value.into(),
			line: 0,
			column: 0,
			error_level: level,
			file_name: String::new(),
		}
	}

	pub fn with_file(mut self, file: impl Into<String>) -> Self {
		self.file_name = file.into();
		self
	}

	pub fn with_level(mut self, level: ErrorLevel) -> Self {
		self.error_level = level;
		self
	}

	pub fn with_line_col(mut self, line: usize, col: usize) -> Self {
		self.line = line;
		self.column = col;
		self
	}

	pub fn message_with_file(message: ManifestErrorId, file: impl Into<String>) -> Self {
		Self::new(message).with_file(file)
	}

	pub fn message_context_with_file(
		message: ManifestErrorId,
		context: impl Into<String>,
		file: impl Into<String>,
	) -> Self {
		Self::with_context(message, context).with_file(file)
	}

	pub fn message_context_value_with_file(
		message: ManifestErrorId,
		context: impl Into<String>,
		value: impl Into<String>,
		file: impl Into<String>,
	) -> Self {
		Self::with_context_value(message, context, value).with_file(file)
	}

	pub fn message_level_with_file(
		message: ManifestErrorId,
		level: ErrorLevel,
		file: impl Into<String>,
	) -> Self {
		Self::new(message).with_level(level).with_file(file)
	}

	pub fn message_context_value_line_level_with_file(
		message: ManifestErrorId,
		context: impl Into<String>,
		value: impl Into<String>,
		line: usize,
		column: usize,
		level: ErrorLevel,
		file: impl Into<String>,
	) -> Self {
		Self {
			message,
			context: context.into(),
			value: value.into(),
			line,
			column,
			error_level: level,
			file_name: file.into(),
		}
	}
}

pub const ERROR_MESSAGE_PREFIX: &str = "Manifest Error: ";
pub const WARNING_MESSAGE_PREFIX: &str = "Manifest Warning: ";

pub fn format_error_message(errors: &[ValidationError]) -> String {
	let mut result = String::new();
	for error in errors {
		match error.error_level {
			ErrorLevel::Error => result.push_str(ERROR_MESSAGE_PREFIX),
			ErrorLevel::Warning => result.push_str(WARNING_MESSAGE_PREFIX),
		}
		result.push_str(error.message.message());
		if !error.context.is_empty() {
			result.push_str(" [");
			result.push_str(&error.context);
			result.push(']');
		}
		if !error.value.is_empty() {
			result.push_str(" Value: ");
			result.push_str(&error.value);
		}
		if error.line > 0 && error.column > 0 {
			result.push_str(&format!(" Line: {}, Column: {}", error.line, error.column));
		}
		if !error.file_name.is_empty() {
			result.push_str(" File: ");
			result.push_str(&error.file_name);
		}
		result.push('\n');
	}
	result
}

pub fn format_error_json(errors: &[ValidationError], full_message: &str) -> String {
	let is_syntax_error = errors.is_empty();
	let errors_array: Vec<serde_json::Value> = errors
		.iter()
		.map(|e| {
			serde_json::json!({
				"errorId": e.message.as_str(),
				"message": e.message.message(),
				"context": e.context,
				"value": e.value,
				"line": e.line,
				"column": e.column,
				"level": match e.error_level {
					ErrorLevel::Error => "Error",
					ErrorLevel::Warning => "Warning",
				},
				"file": e.file_name,
			})
		})
		.collect();

	let root = serde_json::json!({
		"fullMessage": full_message,
		"isSyntaxError": is_syntax_error,
		"errors": errors_array,
	});

	serde_json::to_string(&root).unwrap_or_default()
}

impl fmt::Display for ValidationError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", self.message.message())
	}
}

#[derive(Debug)]
pub struct ManifestException {
	pub errors: Vec<ValidationError>,
	pub syntax_error: Option<String>,
	warning_only: bool,
}

impl ManifestException {
	pub fn new(errors: Vec<ValidationError>) -> Self {
		let warning_only =
			!errors.is_empty() && errors.iter().all(|e| e.error_level == ErrorLevel::Warning);
		Self {
			errors,
			syntax_error: None,
			warning_only,
		}
	}

	pub fn syntax(msg: String) -> Self {
		Self {
			errors: Vec::new(),
			syntax_error: Some(msg),
			warning_only: false,
		}
	}

	pub fn is_warning_only(&self) -> bool {
		self.warning_only
	}

	pub fn error_message(&self) -> String {
		if self.errors.is_empty() {
			self.syntax_error.clone().unwrap_or_default()
		} else {
			format_error_message(&self.errors)
		}
	}

	pub fn error_json(&self) -> String {
		let full = self.error_message();
		format_error_json(&self.errors, &full)
	}
}

impl fmt::Display for ManifestException {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", self.error_message())
	}
}

impl std::error::Error for ManifestException {}
