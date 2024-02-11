use std::process::Command;
fn main() {
    // Git commit hash
    let git_output = Command::new("git").args(&["rev-parse", "HEAD"]).output().unwrap();
    let git_hash = String::from_utf8(git_output.stdout).unwrap();
    let git_hash = git_hash.trim();
    let short_sha: String = git_hash.chars().take(8).collect();
    println!("cargo:rustc-env=GIT_HASH={}", short_sha);

    // Build date
    let date_output = Command::new("date").args(&["--rfc-3339=seconds"]).output().unwrap();
    let date = String::from_utf8(date_output.stdout).unwrap();
    println!("cargo:rustc-env=BUILD_DATE={}", date);

    // Semantic version (VERSION file)
    let semver_output = Command::new("cat").arg("VERSION").output().unwrap();
    let semver = String::from_utf8(semver_output.stdout).unwrap();
    println!("cargo:rustc-env=VERSION={}", semver);
}