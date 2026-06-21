//! Workspace shape guard — verifies Cargo workspace members match directories.

use std::collections::HashSet;
use std::fs;
use std::path::Path;

fn main() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(|p| p.parent())
        .expect("cannot find repo root");

    let mut failures: Vec<String> = Vec::new();

    // Read Cargo workspace members
    let cargo_toml = fs::read_to_string(root.join("Cargo.toml")).expect("cannot read Cargo.toml");
    let cargo_member_list = parse_cargo_member_list(&cargo_toml);
    let cargo_members: HashSet<String> = cargo_member_list.iter().cloned().collect();
    let mut seen_members = HashSet::new();
    for member in &cargo_member_list {
        if !seen_members.insert(member.clone()) {
            failures.push(format!(
                "Cargo workspace members contains duplicate {member}"
            ));
        }
    }

    // Check crates/
    let crates_dir = root.join("crates");
    if crates_dir.exists() {
        for entry in fs::read_dir(&crates_dir).expect("cannot read crates/") {
            let entry = entry.expect("cannot read entry");
            if !entry.file_type().unwrap().is_dir() {
                continue;
            }
            let name = entry.file_name().to_string_lossy().to_string();
            let member = format!("crates/{name}");
            let cargo_path = crates_dir.join(&name).join("Cargo.toml");

            if !cargo_path.exists() {
                failures.push(format!("crates/{name} is missing Cargo.toml"));
                continue;
            }

            if !cargo_members.contains(&member) {
                failures.push(format!("Cargo workspace members is missing {member}"));
            }

            let expected_name = expected_crate_package_name(&name);
            let pkg_name = read_package_name(&cargo_path);
            if pkg_name.as_deref() != Some(expected_name.as_str()) {
                failures.push(format!(
                    "crates/{name}/Cargo.toml name is {pkg_name:?}, expected {expected_name}"
                ));
            }
        }
    }

    // Check apps/
    let apps_dir = root.join("apps");
    if apps_dir.exists() {
        for entry in fs::read_dir(&apps_dir).expect("cannot read apps/") {
            let entry = entry.expect("cannot read entry");
            if !entry.file_type().unwrap().is_dir() {
                continue;
            }
            let name = entry.file_name().to_string_lossy().to_string();
            let cargo_path = apps_dir.join(&name).join("src-tauri").join("Cargo.toml");

            if !cargo_path.exists() {
                continue; // Not a Tauri app
            }

            let member = format!("apps/{name}/src-tauri");
            if !cargo_members.contains(&member) {
                failures.push(format!("Cargo workspace members is missing {member}"));
            }

            let expected_name = format!("tench-{name}");
            let pkg_name = read_package_name(&cargo_path);
            if pkg_name.as_deref() != Some(expected_name.as_str()) {
                failures.push(format!(
                    "apps/{name}/src-tauri/Cargo.toml name is {pkg_name:?}, expected {expected_name}"
                ));
            }
        }
    }

    if failures.is_empty() {
        let crates_count = count_dirs(&crates_dir);
        let apps_count = count_dirs(&apps_dir);
        println!("workspace shape: ok ({apps_count} apps, 0 packages, {crates_count} crates)");
    } else {
        for failure in &failures {
            eprintln!("workspace shape failed: {failure}");
        }
        std::process::exit(1);
    }
}

fn parse_cargo_member_list(content: &str) -> Vec<String> {
    let mut members = HashSet::new();
    let mut ordered = Vec::new();
    let mut bracket_depth = 0;

    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("members") && trimmed.contains('=') {
            bracket_depth = count_brackets(trimmed);
            extract_members(trimmed, &mut members, &mut ordered);
            if bracket_depth == 0 {
                break;
            }
            continue;
        }
        if bracket_depth > 0 {
            bracket_depth += count_brackets(trimmed);
            extract_members(trimmed, &mut members, &mut ordered);
            if bracket_depth == 0 {
                break;
            }
        }
    }

    ordered
}

fn count_brackets(s: &str) -> i32 {
    let mut depth = 0;
    let mut in_string = false;
    for ch in s.chars() {
        if ch == '"' {
            in_string = !in_string;
        }
        if !in_string {
            if ch == '[' {
                depth += 1;
            } else if ch == ']' {
                depth -= 1;
            }
        }
    }
    depth
}

fn extract_members(s: &str, members: &mut HashSet<String>, ordered: &mut Vec<String>) {
    let mut in_quote = false;
    let mut current = String::new();
    for ch in s.chars() {
        if ch == '"' {
            if in_quote {
                if current.contains('/') || current.contains('\\') {
                    let member = current.replace('\\', "/");
                    members.insert(member.clone());
                    ordered.push(member);
                }
                current.clear();
            }
            in_quote = !in_quote;
        } else if in_quote {
            current.push(ch);
        }
    }
}

fn expected_crate_package_name(directory_name: &str) -> String {
    match directory_name {
        "tench-ui" => "tench-ui".to_string(),
        "tench-ui-test" => "tench-ui-test".to_string(),
        _ => format!("tench-{directory_name}"),
    }
}

fn read_package_name(path: &Path) -> Option<String> {
    let content = fs::read_to_string(path).ok()?;
    for line in content.lines() {
        let trimmed = line.trim();
        if let Some(rest) = trimmed.strip_prefix("name") {
            if let Some(rest) = rest.trim_start().strip_prefix('=') {
                let rest = rest.trim().trim_end_matches(',');
                return Some(rest.trim_matches('"').to_string());
            }
        }
    }
    None
}

fn count_dirs(path: &Path) -> usize {
    if !path.exists() {
        return 0;
    }
    fs::read_dir(path)
        .unwrap()
        .filter(|e| {
            e.as_ref()
                .map(|e| e.file_type().unwrap().is_dir())
                .unwrap_or(false)
        })
        .count()
}
