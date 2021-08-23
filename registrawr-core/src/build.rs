use std::io::{stdout, Write};
use std::path::{Path, PathBuf};
use std::process::Command;

pub(crate) fn build(project_path: &Path) -> PathBuf {
    println!("Building: {}", project_path.display());
    run_build(project_path);

    let mut build_path = project_path.to_owned();
    build_path.push("build");

    build_path
}

#[cfg(target_family = "unix")]
fn run_build(project_path: &Path) {
    let output = Command::new("npm")
        .arg("run")
        .arg("build")
        .current_dir(project_path)
        .output()
        .expect("Error running npm build");

    let mut out = stdout();
    out.write_all(&output.stdout)
        .expect("Failed to write to stdout");
}

#[cfg(target_os = "windows")]
fn run_build(project_path: &Path) {
    let output = Command::new("C:\\Windows\\System32\\WindowsPowerShell\\v1.0\\powershell.exe")
        .arg("npm")
        .arg("run")
        .arg("build")
        .current_dir(project_path)
        .output()
        .expect("Error running command");

    let mut out = stdout();
    out.write_all(&output.stdout)
        .expect("Failed to write to stdout");
}
