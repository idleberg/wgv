use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};

pub struct ManifestPathIter {
	seen: HashSet<PathBuf>,
	file_parents: Vec<PathBuf>,
	dir_stack: Vec<PathBuf>,
}

impl Iterator for ManifestPathIter {
	type Item = PathBuf;

	fn next(&mut self) -> Option<PathBuf> {
		while let Some(parent) = self.file_parents.pop() {
			if self.seen.insert(parent.clone()) {
				return Some(parent);
			}
		}

		while let Some(dir) = self.dir_stack.pop() {
			let Ok(entries) = fs::read_dir(&dir) else {
				continue;
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
				if self.seen.insert(dir.clone()) {
					return Some(dir);
				}
				continue;
			}

			subdirs.sort_by(|a, b| b.cmp(a));
			self.dir_stack.extend(subdirs);
		}

		None
	}
}

pub fn discover_manifest_paths(inputs: &[String]) -> Result<ManifestPathIter, String> {
	let mut file_parents: Vec<PathBuf> = Vec::new();
	let mut dir_stack: Vec<PathBuf> = Vec::new();

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
				dir_stack.push(path);
			} else {
				file_parents.push(path.parent().unwrap_or(Path::new(".")).to_path_buf());
			}
		} else {
			for path in matches {
				if path.is_dir() {
					dir_stack.push(path);
				} else {
					file_parents.push(path.parent().unwrap_or(Path::new(".")).to_path_buf());
				}
			}
		}
	}

	dir_stack.sort_by(|a, b| b.cmp(a));

	Ok(ManifestPathIter {
		seen: HashSet::new(),
		file_parents,
		dir_stack,
	})
}
