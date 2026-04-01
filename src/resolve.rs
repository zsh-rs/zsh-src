use std::io;
use std::process::Command;

use crate::cache::CacheDir;
use crate::headers::ZshSource;

#[cfg(feature = "download")]
pub fn resolve() -> io::Result<ZshSource> {
    let version = get_zsh_version();

    Ok(CacheDir::new()
        .download(&version)?
        .extract()?
        .ensure_headers())
}

fn get_zsh_version() -> String {
    let output = Command::new("zsh")
        .arg("-fc")
        .arg("'echo $ZSH_VERSION'")
        .output()
        .expect("failed to run zsh");

    String::from_utf8(output.stdout).unwrap()
    // let stdout = String::from_utf8(output.stdout).unwrap();

    // Example: "zsh 5.9 (x86_64-pc-linux-gnu)"
    // stdout
    //     .split_whitespace()
    //     .nth(1)
    //     .expect("failed to parse zsh version")
    //     .to_string()
}
