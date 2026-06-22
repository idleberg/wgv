use yaml_rust2::Yaml;

pub fn yaml_get_str(node: &Yaml, key: &str) -> Option<String> {
	match node {
		Yaml::Hash(map) => {
			for (k, v) in map {
				if let Yaml::String(ks) = k
					&& ks == key
				{
					return match v {
						Yaml::String(s) => Some(s.to_string()),
						Yaml::Integer(i) => Some(i.to_string()),
						Yaml::Real(s) => Some(s.clone()),
						Yaml::Boolean(b) => Some(b.to_string()),
						_ => None,
					};
				}
			}
			None
		}
		_ => None,
	}
}

pub fn yaml_has_key(node: &Yaml, key: &str) -> bool {
	match node {
		Yaml::Hash(map) => map.iter().any(|(k, _)| {
			if let Yaml::String(ks) = k {
				ks == key
			} else {
				false
			}
		}),
		_ => false,
	}
}

pub fn yaml_get_array<'a>(node: &'a Yaml, key: &str) -> Option<&'a [Yaml]> {
	match node {
		Yaml::Hash(map) => {
			for (k, v) in map {
				if let Yaml::String(ks) = k
					&& ks == key && let Yaml::Array(arr) = v
				{
					return Some(arr);
				}
			}
			None
		}
		_ => None,
	}
}

pub fn yaml_get_map<'a>(node: &'a Yaml, key: &str) -> Option<&'a Yaml> {
	match node {
		Yaml::Hash(map) => {
			for (k, v) in map {
				if let Yaml::String(ks) = k
					&& ks == key
				{
					return Some(v);
				}
			}
			None
		}
		_ => None,
	}
}
