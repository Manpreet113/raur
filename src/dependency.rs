use std::process::{Command, Stdio};

pub enum PackageOrigin {
    Repo,
    Aur,
}

pub fn clean_dependency(dep: &str) -> String {
    if let Some(idx) = dep.find(|c| c == '>' || c == '<' || c == '=') {
        return dep[..idx].to_string();
    }
    dep.to_string()
}

pub fn check_origin(pkg_name: &str) -> PackageOrigin{

    // TODO: Use pacman -Ssq and pass a Vec<&str> to avoid so many syscalls

    let status = Command::new("pacman")
        .arg("-Si")
        .arg(pkg_name)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .expect("Failed to run pacman check");

    if status.success() {
        PackageOrigin::Repo
    } else {
        PackageOrigin::Aur
    }
}