use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};

pub fn resolve_manifest_paths(inputs: &[String]) -> Result<Vec<PathBuf>, String> {
	let mut file_paths: Vec<PathBuf> = Vec::new();
	let mut dir_paths: Vec<PathBuf> = Vec::new();

	for input in inputs {
		let pattern =
			glob::glob(input).map_err(|e| format!("Invalid glob pattern '{input}': {e}"))?;

		let matches: Vec<PathBuf> = pattern.filter_map(|r| r.ok()).collect();

		if matches.is_empty() {
			let path = PathBuf::from(input);
			if !path.exists() {
				return Err(format!("Path does not exist: {input}"));
			}
			if path.is_dir() {
				dir_paths.push(path);
			} else {
				file_paths.push(path);
			}
		} else {
			for path in matches {
				if path.is_dir() {
					dir_paths.push(path);
				} else {
					file_paths.push(path);
				}
			}
		}
	}

	let mut result_set: HashSet<PathBuf> = HashSet::new();
	let mut result: Vec<PathBuf> = Vec::new();

	for file in &file_paths {
		let parent = file.parent().unwrap_or(Path::new(".")).to_path_buf();

		if result_set.insert(parent.clone()) {
			result.push(parent);
		}
	}

	for dir in &dir_paths {
		let manifest_dirs = detect_manifest_dirs(dir);
		if manifest_dirs.is_empty() {
			if result_set.insert(dir.clone()) {
				result.push(dir.clone());
			}
		} else {
			for md in manifest_dirs {
				if result_set.insert(md.clone()) {
					result.push(md);
				}
			}
		}
	}

	if result.is_empty() {
		return Err("No manifests found".to_string());
	}

	result.sort();
	Ok(result)
}

fn detect_manifest_dirs(root: &Path) -> Vec<PathBuf> {
	let mut result = Vec::new();

	let Ok(entries) = fs::read_dir(root) else {
		return result;
	};

	let mut has_yaml = false;
	let mut subdirs: Vec<PathBuf> = Vec::new();

	for entry in entries.filter_map(|e| e.ok()) {
		let path = entry.path();
		if path.is_dir() {
			subdirs.push(path);
		} else if !has_yaml {
			has_yaml = matches!(
				path.extension().and_then(|ext| ext.to_str()),
				Some("yaml" | "yml")
			);
		}
	}

	if has_yaml && subdirs.is_empty() {
		result.push(root.to_path_buf());
		return result;
	}

	subdirs.sort();
	for subdir in subdirs {
		result.extend(detect_manifest_dirs(&subdir));
	}

	result
}
