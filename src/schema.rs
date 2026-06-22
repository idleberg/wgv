use crate::common::{ManifestTypeEnum, ManifestVer};

macro_rules! include_schema {
	($dir:literal, $ver:literal, $type:literal) => {
		include_str!(concat!(
			"../schemas/",
			$dir,
			"/manifest.",
			$type,
			".",
			$ver,
			".json"
		))
	};
}

struct SchemaSet {
	singleton: &'static str,
	version: &'static str,
	installer: &'static str,
	default_locale: &'static str,
	locale: &'static str,
}

const SCHEMAS_V1_28: SchemaSet = SchemaSet {
	singleton: include_schema!("v1.28.0", "1.28.0", "singleton"),
	version: include_schema!("v1.28.0", "1.28.0", "version"),
	installer: include_schema!("v1.28.0", "1.28.0", "installer"),
	default_locale: include_schema!("v1.28.0", "1.28.0", "defaultLocale"),
	locale: include_schema!("v1.28.0", "1.28.0", "locale"),
};

const SCHEMAS_V1_12: SchemaSet = SchemaSet {
	singleton: include_schema!("v1.12.0", "1.12.0", "singleton"),
	version: include_schema!("v1.12.0", "1.12.0", "version"),
	installer: include_schema!("v1.12.0", "1.12.0", "installer"),
	default_locale: include_schema!("v1.12.0", "1.12.0", "defaultLocale"),
	locale: include_schema!("v1.12.0", "1.12.0", "locale"),
};

const SCHEMAS_V1_10: SchemaSet = SchemaSet {
	singleton: include_schema!("v1.10.0", "1.10.0", "singleton"),
	version: include_schema!("v1.10.0", "1.10.0", "version"),
	installer: include_schema!("v1.10.0", "1.10.0", "installer"),
	default_locale: include_schema!("v1.10.0", "1.10.0", "defaultLocale"),
	locale: include_schema!("v1.10.0", "1.10.0", "locale"),
};

const SCHEMAS_V1_9: SchemaSet = SchemaSet {
	singleton: include_schema!("v1.9.0", "1.9.0", "singleton"),
	version: include_schema!("v1.9.0", "1.9.0", "version"),
	installer: include_schema!("v1.9.0", "1.9.0", "installer"),
	default_locale: include_schema!("v1.9.0", "1.9.0", "defaultLocale"),
	locale: include_schema!("v1.9.0", "1.9.0", "locale"),
};

const SCHEMAS_V1_7: SchemaSet = SchemaSet {
	singleton: include_schema!("v1.7.0", "1.7.0", "singleton"),
	version: include_schema!("v1.7.0", "1.7.0", "version"),
	installer: include_schema!("v1.7.0", "1.7.0", "installer"),
	default_locale: include_schema!("v1.7.0", "1.7.0", "defaultLocale"),
	locale: include_schema!("v1.7.0", "1.7.0", "locale"),
};

const SCHEMAS_V1_6: SchemaSet = SchemaSet {
	singleton: include_schema!("v1.6.0", "1.6.0", "singleton"),
	version: include_schema!("v1.6.0", "1.6.0", "version"),
	installer: include_schema!("v1.6.0", "1.6.0", "installer"),
	default_locale: include_schema!("v1.6.0", "1.6.0", "defaultLocale"),
	locale: include_schema!("v1.6.0", "1.6.0", "locale"),
};

const SCHEMAS_V1_5: SchemaSet = SchemaSet {
	singleton: include_schema!("v1.5.0", "1.5.0", "singleton"),
	version: include_schema!("v1.5.0", "1.5.0", "version"),
	installer: include_schema!("v1.5.0", "1.5.0", "installer"),
	default_locale: include_schema!("v1.5.0", "1.5.0", "defaultLocale"),
	locale: include_schema!("v1.5.0", "1.5.0", "locale"),
};

const SCHEMAS_V1_4: SchemaSet = SchemaSet {
	singleton: include_schema!("v1.4.0", "1.4.0", "singleton"),
	version: include_schema!("v1.4.0", "1.4.0", "version"),
	installer: include_schema!("v1.4.0", "1.4.0", "installer"),
	default_locale: include_schema!("v1.4.0", "1.4.0", "defaultLocale"),
	locale: include_schema!("v1.4.0", "1.4.0", "locale"),
};

const SCHEMAS_V1_2: SchemaSet = SchemaSet {
	singleton: include_schema!("v1.2.0", "1.2.0", "singleton"),
	version: include_schema!("v1.2.0", "1.2.0", "version"),
	installer: include_schema!("v1.2.0", "1.2.0", "installer"),
	default_locale: include_schema!("v1.2.0", "1.2.0", "defaultLocale"),
	locale: include_schema!("v1.2.0", "1.2.0", "locale"),
};

const SCHEMAS_V1_1: SchemaSet = SchemaSet {
	singleton: include_schema!("v1.1.0", "1.1.0", "singleton"),
	version: include_schema!("v1.1.0", "1.1.0", "version"),
	installer: include_schema!("v1.1.0", "1.1.0", "installer"),
	default_locale: include_schema!("v1.1.0", "1.1.0", "defaultLocale"),
	locale: include_schema!("v1.1.0", "1.1.0", "locale"),
};

const SCHEMAS_V1_0: SchemaSet = SchemaSet {
	singleton: include_schema!("v1.0.0", "1.0.0", "singleton"),
	version: include_schema!("v1.0.0", "1.0.0", "version"),
	installer: include_schema!("v1.0.0", "1.0.0", "installer"),
	default_locale: include_schema!("v1.0.0", "1.0.0", "defaultLocale"),
	locale: include_schema!("v1.0.0", "1.0.0", "locale"),
};

fn ver(s: &str) -> ManifestVer {
	ManifestVer::parse(s).unwrap()
}

fn select_schema_set(manifest_version: &ManifestVer) -> &'static SchemaSet {
	if *manifest_version >= ver("1.28.0") {
		&SCHEMAS_V1_28
	} else if *manifest_version >= ver("1.12.0") {
		&SCHEMAS_V1_12
	} else if *manifest_version >= ver("1.10.0") {
		&SCHEMAS_V1_10
	} else if *manifest_version >= ver("1.9.0") {
		&SCHEMAS_V1_9
	} else if *manifest_version >= ver("1.7.0") {
		&SCHEMAS_V1_7
	} else if *manifest_version >= ver("1.6.0") {
		&SCHEMAS_V1_6
	} else if *manifest_version >= ver("1.5.0") {
		&SCHEMAS_V1_5
	} else if *manifest_version >= ver("1.4.0") {
		&SCHEMAS_V1_4
	} else if *manifest_version >= ver("1.2.0") {
		&SCHEMAS_V1_2
	} else if *manifest_version >= ver("1.1.0") {
		&SCHEMAS_V1_1
	} else {
		&SCHEMAS_V1_0
	}
}

pub fn load_schema_str(
	manifest_version: &ManifestVer,
	manifest_type: ManifestTypeEnum,
) -> Option<&'static str> {
	let set = select_schema_set(manifest_version);
	match manifest_type {
		ManifestTypeEnum::Singleton => Some(set.singleton),
		ManifestTypeEnum::Version => Some(set.version),
		ManifestTypeEnum::Installer => Some(set.installer),
		ManifestTypeEnum::DefaultLocale => Some(set.default_locale),
		ManifestTypeEnum::Locale => Some(set.locale),
		_ => None,
	}
}

pub fn load_schema_json(
	manifest_version: &ManifestVer,
	manifest_type: ManifestTypeEnum,
) -> Option<serde_json::Value> {
	let s = load_schema_str(manifest_version, manifest_type)?;
	serde_json::from_str(s).ok()
}
