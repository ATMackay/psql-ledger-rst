use std::fs::File;
use std::io::Read;
use std::process::Command;
use toml::Value;

struct Project {
    name: String,
    version: String,
}

fn read_project_info() -> Project {
    let mut file = match File::open("Cargo.toml") {
        Ok(file) => file,
        Err(_) => {
            panic!("failed to open Cargo.toml")
        }
    };

    let mut file_contents = String::new();
    if let Err(_) = file.read_to_string(&mut file_contents) {
        panic!("failed to read Cargo.toml");
    };

    let value: Value = match file_contents.parse() {
        Ok(value) => value,
        Err(_) => {
            panic!("failed to parse Cargo.toml")
        }
    };

    let version = match value.get("package") {
        Some(package) => match package.get("version") {
            Some(version) => match version.as_str() {
                Some(version_str) => version_str.to_string(),
                None => panic!("version field in Cargo.toml not a string"),
            },
            None => panic!("version field not in Cargo.toml"),
        },
        None => panic!("[package] section not found in Config.toml"),
    };

    let project_name = match value.get("package") {
        Some(package) => match package.get("name") {
            Some(project_name) => match project_name.as_str() {
                Some(project_name_str) => project_name_str.to_string(),
                None => panic!("name field in Cargo.toml not a string"),
            },
            None => panic!("name field not in Cargo.toml"),
        },
        None => panic!("[package] section not found in Config.toml"),
    };

    let p: Project = Project {
        name: project_name,
        version: version,
    };

    p
}

fn main() {
    // Git commit hash
    let git_output = Command::new("git")
        .args(&["rev-parse", "HEAD"])
        .output()
        .unwrap();
    let git_hash = String::from_utf8(git_output.stdout).unwrap();
    let git_hash = git_hash.trim();
    let short_sha: String = git_hash.chars().take(8).collect();
    println!("cargo:rustc-env=GIT_HASH={}", short_sha);

    // Build date
    let date_output = Command::new("date")
        .args(&["--rfc-3339=seconds"])
        .output()
        .unwrap();
    let date = String::from_utf8(date_output.stdout).unwrap();
    println!("cargo:rustc-env=BUILD_DATE={}", date);

    // Read Cargo.toml to get semantic version and project name
    let project_info = read_project_info();
    let semver = project_info.version;
    let service_name = project_info.name;

    println!("cargo:rustc-env=VERSION={}", semver);
    println!("cargo:rustc-env=SERVICENAME={}", service_name);
}
