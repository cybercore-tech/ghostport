//! The binary reports its real package version. v0.1.1 and v0.1.2 printed "ghostport 0.1.0"
//! because the version was a hardcoded string.

#[test]
fn version_matches_the_package() {
    let output = std::process::Command::new(env!("CARGO_BIN_EXE_ghostport"))
        .arg("--version")
        .output()
        .expect("run ghostport --version");
    assert!(output.status.success());
    assert_eq!(
        String::from_utf8_lossy(&output.stdout).trim(),
        format!("ghostport {}", env!("CARGO_PKG_VERSION"))
    );
}
