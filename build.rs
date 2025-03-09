use chrono::Utc;
use std::process::Command;

fn main() {
    // Read Cargo.toml to get semantic version and project name
    println!("cargo:rustc-env=VERSION={}", env!("CARGO_PKG_VERSION"));
    println!("cargo:rustc-env=SERVICE_NAME={}", env!("CARGO_PKG_NAME"));

    // Build date
    let build_date = Utc::now().format("%Y-%m-%d %H:%M:%S UTC").to_string();
    println!("cargo:rustc-env=BUILD_DATE={}", build_date);

    // Git commit hash
    let git_output = Command::new("git")
        .args(["rev-parse", "HEAD"])
        .output()
        .unwrap();
    let git_hash = String::from_utf8(git_output.stdout).unwrap();
    let git_hash = git_hash.trim();
    // let short_sha: String = git_hash.chars().take(8).collect();
    println!("cargo:rustc-env=GIT_COMMIT={}", git_hash);

    // Git commit timestamp
    let git_output = Command::new("git")
    .args(["show", "-s", "--format='%ci'", git_hash])
    .output()
    .unwrap();
    let git_timestamp = String::from_utf8(git_output.stdout).unwrap();
    let git_timestamp = git_timestamp.trim();
    println!("cargo:rustc-env=GIT_COMMIT_DATE={}", git_timestamp);

    // Git semver
    let git_output = Command::new("git")
        .args(["describe", "--tags"])
        .output()
        .unwrap();
    let git_ver = String::from_utf8(git_output.stdout).unwrap();
    println!("cargo:rustc-env=GIT_VERSION_TAG={}", git_ver);
}
