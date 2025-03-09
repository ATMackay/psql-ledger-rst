use chrono::Utc;
use std::process::Command;

fn main() {
    // Git commit hash
    let git_output = Command::new("git")
        .args(&["rev-parse", "HEAD"])
        .output()
        .unwrap();
    let git_hash = String::from_utf8(git_output.stdout).unwrap();
    let git_hash = git_hash.trim();
    let short_sha: String = git_hash.chars().take(8).collect();
    println!("cargo:rustc-env=GIT_COMMIT={}", short_sha);

    // Build date
    let build_date = Utc::now().format("%Y-%m-%d %H:%M:%S UTC").to_string();
    println!("cargo:rustc-env=BUILD_DATE={}", build_date);

    // Read Cargo.toml to get semantic version and project name
    println!("cargo:rustc-env=VERSION={}", env!("CARGO_PKG_VERSION"));
    println!("cargo:rustc-env=SERVICE_NAME={}", env!("CARGO_PKG_NAME"));
}
