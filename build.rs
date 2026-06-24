use std::{
    process::Command,
    time::{SystemTime, UNIX_EPOCH},
};

fn main() {
    add_build_script_inputs();
    add_git_revision();
    add_build_timestamp();
}

fn add_build_script_inputs() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=Cargo.toml");
    println!("cargo:rerun-if-changed=src");
}

fn add_git_revision() {
    add_git_revision_inputs();

    let rev =
        git_output(&["rev-parse", "--short", "HEAD"]).unwrap_or_else(|| "unknown".to_string());

    println!("cargo:rustc-env=GIT_REVISION={rev}");
}

fn add_git_revision_inputs() {
    if let Some(head_path) = git_output(&["rev-parse", "--git-path", "HEAD"]) {
        println!("cargo:rerun-if-changed={head_path}");
    }

    if let Some(head_ref) = git_output(&["symbolic-ref", "--quiet", "HEAD"]) {
        if let Some(head_ref_path) = git_output(&["rev-parse", "--git-path", &head_ref]) {
            println!("cargo:rerun-if-changed={head_ref_path}");
        }
    }
}

fn git_output(args: &[&str]) -> Option<String> {
    Command::new("git")
        .args(args)
        .output()
        .ok()
        .and_then(|o| o.status.success().then_some(o.stdout))
        .and_then(|v| String::from_utf8(v).ok())
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
}

fn add_build_timestamp() {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock should be after Unix epoch")
        .as_secs();

    println!("cargo:rustc-env=BUILD_TIMESTAMP_UNIX={timestamp}");
}
