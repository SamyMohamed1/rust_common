use std::process::Command;

use regex::Regex;

#[derive(Debug)]
pub struct Version(pub u32, pub u32, pub u32);

fn main() {
    let Some(v) = rustc_version() else {
        println!("cargo::warning=Cannot get rustc version");
        return;
    };
    if v.0 > 1 || (v.0.eq(&1) && v.1.ge(&81)) {
        println!("cargo:rustc-cfg=feature=\"error_in_core\"");
    }
}

fn rustc_version() -> Option<Version> {
    let rustc = std::env::var("RUSTC").ok()?;
    let output = Command::new(rustc).arg("--version").output().ok()?;
    let version = std::str::from_utf8(&output.stdout).ok()?;
    let regex = Regex::new(r"rustc +([0-9]+)\.([0-9]+)\.([0-9]+).*").ok()?;
    let caps = regex.captures(version)?;
    let major: u32 = caps.get(1)?.as_str().parse().ok()?;
    let minor: u32 = caps.get(2)?.as_str().parse().ok()?;
    let patch: u32 = caps.get(3)?.as_str().parse().ok()?;
    Some(Version(major, minor, patch))
}
