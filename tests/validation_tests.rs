use std::path::PathBuf;
use wgv::common::*;
use wgv::error::ManifestErrorId;
use wgv::manifest;

fn fixtures() -> PathBuf {
	PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures")
}

fn default_option() -> ManifestValidateOption {
	ManifestValidateOption {
		full_validation: true,
		schema_header_validation_as_warning: true,
		throw_on_warning: false,
		..Default::default()
	}
}

fn strict_option() -> ManifestValidateOption {
	ManifestValidateOption {
		full_validation: true,
		schema_header_validation_as_warning: true,
		throw_on_warning: true,
		..Default::default()
	}
}

fn validate(dir: &str) -> Result<(), wgv::error::ManifestException> {
	manifest::validate_from_path(&fixtures().join(dir), &default_option())
}

fn validate_strict(dir: &str) -> Result<(), wgv::error::ManifestException> {
	manifest::validate_from_path(&fixtures().join(dir), &strict_option())
}

// --- Valid manifests ---

#[test]
fn valid_singleton() {
	validate("valid_singleton").expect("valid singleton should pass");
}

#[test]
fn valid_multifile() {
	validate("valid_multifile").expect("valid multi-file manifest should pass");
}

// --- Bug fix: root-level field inheritance ---

#[test]
fn root_level_installer_type_inheritance() {
	validate("root_inheritance").expect("installers should inherit InstallerType from root level");
}

// --- Bug fix: YAML numeric version coercion ---

#[test]
fn numeric_package_version() {
	validate("numeric_version").expect("numeric PackageVersion (1.0) should be coerced to string");
}

// --- Bug fix: Portable + ProductCode ---

#[test]
fn portable_with_product_code() {
	validate("portable_productcode").expect("portable installer with ProductCode should be valid");
}

// --- Bug fix: archive + different NestedInstallerTypes not duplicate ---

#[test]
fn archive_different_nested_types_not_duplicate() {
	validate("archive_nested_types")
		.expect("zip+portable and zip+wix with same arch should not be duplicates");
}

// --- Bug fix: schema header on line 2 ---

#[test]
fn schema_header_after_tool_comment() {
	validate("schema_header_line2")
		.expect("schema header on line 2 after tool comment should be found");
}

// --- Error cases ---

#[test]
fn duplicate_installer_detected() {
	let err = validate("duplicate_installers").expect_err("duplicate installers should fail");
	assert!(
		err.errors
			.iter()
			.any(|e| e.message == ManifestErrorId::DuplicateInstallerEntry),
		"should contain DuplicateInstallerEntry error"
	);
}

#[test]
fn duplicate_archive_same_nested_type() {
	let err =
		validate("invalid_duplicate_archive").expect_err("same nested type duplicates should fail");
	assert!(
		err.errors
			.iter()
			.any(|e| e.message == ManifestErrorId::DuplicateInstallerEntry),
		"should detect duplicate archive installers with same NestedInstallerType"
	);
}

#[test]
fn missing_required_field() {
	let err = validate("missing_required").expect_err("missing PackageVersion should fail");
	assert!(
		err.errors
			.iter()
			.any(|e| e.message == ManifestErrorId::RequiredFieldMissing),
		"should contain RequiredFieldMissing error"
	);
}

#[test]
fn exe_missing_silent_switches_warning() {
	let result_lenient = validate("exe_missing_switches");
	assert!(
		result_lenient.is_ok(),
		"missing switches should pass with throw_on_warning=false"
	);

	let err = validate_strict("exe_missing_switches")
		.expect_err("missing switches should fail with throw_on_warning=true");
	assert!(
		err.is_warning_only(),
		"missing switches should be warning-only"
	);
	assert!(
		err.errors
			.iter()
			.any(|e| e.message == ManifestErrorId::ExeInstallerMissingSilentSwitches),
		"should contain ExeInstallerMissingSilentSwitches warning"
	);
}

// --- Unit tests for common types ---

#[test]
fn installer_type_parsing() {
	assert_eq!(InstallerType::parse_str("exe"), InstallerType::Exe);
	assert_eq!(InstallerType::parse_str("EXE"), InstallerType::Exe);
	assert_eq!(InstallerType::parse_str("msi"), InstallerType::Msi);
	assert_eq!(InstallerType::parse_str("appx"), InstallerType::Msix);
	assert_eq!(InstallerType::parse_str("msix"), InstallerType::Msix);
	assert_eq!(InstallerType::parse_str("zip"), InstallerType::Zip);
	assert_eq!(InstallerType::parse_str("garbage"), InstallerType::Unknown);
}

#[test]
fn uses_product_code_includes_portable() {
	assert!(InstallerType::Portable.uses_product_code());
	assert!(InstallerType::Exe.uses_product_code());
	assert!(InstallerType::Msi.uses_product_code());
	assert!(InstallerType::Inno.uses_product_code());
	assert!(InstallerType::Wix.uses_product_code());
	assert!(InstallerType::Nullsoft.uses_product_code());
	assert!(InstallerType::Burn.uses_product_code());
	assert!(!InstallerType::Zip.uses_product_code());
	assert!(!InstallerType::Msix.uses_product_code());
	assert!(!InstallerType::MSStore.uses_product_code());
}

#[test]
fn is_archive() {
	assert!(InstallerType::Zip.is_archive());
	assert!(!InstallerType::Exe.is_archive());
	assert!(!InstallerType::Msi.is_archive());
}

#[test]
fn manifest_ver_parsing() {
	let v = ManifestVer::parse("1.10.0").unwrap();
	assert_eq!(v.major, 1);
	assert_eq!(v.minor, 10);
	assert_eq!(v.patch, 0);

	let v2 = ManifestVer::parse("1.4.0-preview").unwrap();
	assert_eq!(v2.major, 1);
	assert_eq!(v2.minor, 4);
	assert_eq!(v2.patch, 0);

	assert!(ManifestVer::parse("invalid").is_none());
	assert!(ManifestVer::parse("1.0").is_none());
}

#[test]
fn manifest_ver_ordering() {
	let v1_0 = ManifestVer::new(1, 0, 0);
	let v1_1 = ManifestVer::new(1, 1, 0);
	let v1_10 = ManifestVer::new(1, 10, 0);
	assert!(v1_0 < v1_1);
	assert!(v1_1 < v1_10);
}

#[test]
fn scope_parsing() {
	assert_eq!(ScopeEnum::parse_str("user"), ScopeEnum::User);
	assert_eq!(ScopeEnum::parse_str("Machine"), ScopeEnum::Machine);
	assert_eq!(ScopeEnum::parse_str(""), ScopeEnum::Unknown);
}

#[test]
fn architecture_parsing() {
	assert_eq!(Architecture::parse_str("x64"), Architecture::X64);
	assert_eq!(Architecture::parse_str("X86"), Architecture::X86);
	assert_eq!(Architecture::parse_str("arm64"), Architecture::Arm64);
	assert_eq!(Architecture::parse_str("neutral"), Architecture::Neutral);
	assert_eq!(Architecture::parse_str("sparc"), Architecture::Unknown);
}
