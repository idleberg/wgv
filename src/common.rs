use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InstallerType {
	Unknown,
	Inno,
	Wix,
	Msi,
	Nullsoft,
	Zip,
	Msix,
	Exe,
	Burn,
	MSStore,
	Portable,
	Font,
}

impl InstallerType {
	pub fn parse_str(s: &str) -> Self {
		match s.to_lowercase().as_str() {
			"inno" => Self::Inno,
			"wix" => Self::Wix,
			"msi" => Self::Msi,
			"nullsoft" => Self::Nullsoft,
			"zip" => Self::Zip,
			"msix" | "appx" => Self::Msix,
			"exe" => Self::Exe,
			"burn" => Self::Burn,
			"msstore" => Self::MSStore,
			"portable" => Self::Portable,
			"font" => Self::Font,
			_ => Self::Unknown,
		}
	}

	pub fn as_str(&self) -> &'static str {
		match self {
			Self::Unknown => "Unknown",
			Self::Inno => "Inno",
			Self::Wix => "Wix",
			Self::Msi => "Msi",
			Self::Nullsoft => "Nullsoft",
			Self::Zip => "Zip",
			Self::Msix => "Msix",
			Self::Exe => "Exe",
			Self::Burn => "Burn",
			Self::MSStore => "MSStore",
			Self::Portable => "Portable",
			Self::Font => "Font",
		}
	}

	pub fn is_archive(&self) -> bool {
		matches!(self, Self::Zip)
	}

	pub fn uses_package_family_name(&self) -> bool {
		matches!(self, Self::Msix | Self::MSStore)
	}

	pub fn uses_product_code(&self) -> bool {
		matches!(
			self,
			Self::Inno
				| Self::Wix | Self::Msi
				| Self::Nullsoft
				| Self::Exe | Self::Burn
				| Self::Portable
		)
	}

	pub fn writes_arp_entry(&self) -> bool {
		!matches!(self, Self::Zip | Self::Msix | Self::MSStore | Self::Font)
	}
}

impl fmt::Display for InstallerType {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ManifestTypeEnum {
	Singleton,
	Version,
	Installer,
	DefaultLocale,
	Locale,
	Merged,
	Preview,
	Shadow,
}

impl ManifestTypeEnum {
	pub fn parse_str(s: &str) -> Self {
		match s.to_lowercase().as_str() {
			"singleton" => Self::Singleton,
			"version" => Self::Version,
			"installer" => Self::Installer,
			"defaultlocale" => Self::DefaultLocale,
			"locale" => Self::Locale,
			"merged" => Self::Merged,
			"preview" => Self::Preview,
			"shadow" => Self::Shadow,
			_ => Self::Singleton,
		}
	}

	pub fn as_str(&self) -> &'static str {
		match self {
			Self::Singleton => "singleton",
			Self::Version => "version",
			Self::Installer => "installer",
			Self::DefaultLocale => "defaultLocale",
			Self::Locale => "locale",
			Self::Merged => "merged",
			Self::Preview => "preview",
			Self::Shadow => "shadow",
		}
	}
}

impl fmt::Display for ManifestTypeEnum {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ScopeEnum {
	Unknown,
	User,
	Machine,
}

impl ScopeEnum {
	pub fn parse_str(s: &str) -> Self {
		match s.to_lowercase().as_str() {
			"user" => Self::User,
			"machine" => Self::Machine,
			_ => Self::Unknown,
		}
	}

	pub fn as_str(&self) -> &'static str {
		match self {
			Self::Unknown => "Unknown",
			Self::User => "User",
			Self::Machine => "Machine",
		}
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Architecture {
	Unknown,
	X86,
	X64,
	Arm,
	Arm64,
	Neutral,
}

impl Architecture {
	pub fn parse_str(s: &str) -> Self {
		match s.to_lowercase().as_str() {
			"x86" => Self::X86,
			"x64" => Self::X64,
			"arm" => Self::Arm,
			"arm64" => Self::Arm64,
			"neutral" => Self::Neutral,
			_ => Self::Unknown,
		}
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum UpdateBehavior {
	Unknown,
	Install,
	UninstallPrevious,
	Deny,
}

impl UpdateBehavior {
	pub fn parse_str(s: &str) -> Self {
		match s.to_lowercase().as_str() {
			"install" => Self::Install,
			"uninstallprevious" => Self::UninstallPrevious,
			"deny" => Self::Deny,
			_ => Self::Unknown,
		}
	}
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ManifestVer {
	pub major: u16,
	pub minor: u16,
	pub patch: u16,
}

impl ManifestVer {
	pub fn new(major: u16, minor: u16, patch: u16) -> Self {
		Self {
			major,
			minor,
			patch,
		}
	}

	pub fn parse(s: &str) -> Option<Self> {
		let base = s.split('-').next().unwrap_or(s);
		let parts: Vec<&str> = base.split('.').collect();
		if parts.len() != 3 {
			return None;
		}
		Some(Self {
			major: parts[0].parse().ok()?,
			minor: parts[1].parse().ok()?,
			patch: parts[2].parse().ok()?,
		})
	}
}

impl fmt::Display for ManifestVer {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
	}
}

pub const MAX_SUPPORTED_MAJOR_VERSION: u16 = 1;
pub const DEFAULT_MANIFEST_VERSION: &str = "0.1.0";
pub const MANIFEST_VERSION_V1: &str = "1.0.0";
pub const MANIFEST_VERSION_V1_1: &str = "1.1.0";
pub const MANIFEST_VERSION_V1_2: &str = "1.2.0";
pub const MANIFEST_VERSION_V1_4: &str = "1.4.0";
pub const MANIFEST_VERSION_V1_5: &str = "1.5.0";
pub const MANIFEST_VERSION_V1_6: &str = "1.6.0";
pub const MANIFEST_VERSION_V1_7: &str = "1.7.0";
pub const MANIFEST_VERSION_V1_9: &str = "1.9.0";
pub const MANIFEST_VERSION_V1_10: &str = "1.10.0";
pub const MANIFEST_VERSION_V1_12: &str = "1.12.0";
pub const MANIFEST_VERSION_V1_28: &str = "1.28.0";

#[derive(Default)]
pub struct ManifestValidateOption {
	pub schema_validation_only: bool,
	pub error_on_verified_publisher_fields: bool,
	pub installer_validation: bool,
	pub full_validation: bool,
	pub throw_on_warning: bool,
	pub allow_shadow_manifest: bool,
	pub schema_header_validation_as_warning: bool,
}
