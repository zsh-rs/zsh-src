use directories::ProjectDirs;
use std::path::PathBuf;

const APP_NAME: &str = "zsh-src";
const ORG_NAME: &str = "zsh-rs";
const QUALIFIER: &str = "dev";

pub(crate) struct CacheDir {
    pub(super) root: PathBuf,
}

impl CacheDir {
    pub(crate) fn new() -> Self {
        let proj = ProjectDirs::from(QUALIFIER, ORG_NAME, APP_NAME).expect("no cache dir");

        let dir = proj.cache_dir();

        println!("cache dir: {}", dir.display());

        std::fs::create_dir_all(dir).unwrap();
        CacheDir {
            root: dir.to_path_buf(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cache_dir() {
        let dir = CacheDir::new();
        assert!(dir.root.exists());
    }
}
