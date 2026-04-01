use std::ops::Not;
use std::path::PathBuf;

pub struct ZshSource {
    pub version: String,
    pub source: std::path::PathBuf,
}

impl ZshSource {
    pub(crate) fn new<S: ToString>(version: S, source: PathBuf) -> Self {
        Self {
            version: version.to_string(),
            source,
        }
    }

    #[cfg(feature = "download")]
    pub(crate) fn ensure_headers(self) -> Self {
        let complete_marker = self.source.join(".complete");

        if complete_marker.exists() {
            return self;
        }

        self.source
            .join("./configure")
            .exists()
            .not()
            .then(|| commands::autoreconf(&self.source));

        commands::configure(&self.source);
        commands::make_prep(&self.source);
        commands::make_headers(&self.source);

        std::fs::write(&complete_marker, b"ok").unwrap();

        self
    }
}

#[cfg(feature = "download")]
mod commands {
    use std::path::{Path, PathBuf};
    use std::process::Command;

    /// Generate configure script if submodule doesn't include it
    pub(super) fn autoreconf(src: &Path) {
        Command::new("autoreconf").arg("-fi").current_dir(src).run()
    }

    // Run configure out-of-tree — produces config.h, Makefiles, signames.h, etc.
    pub(super) fn configure(src: &Path) {
        let config_site = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("config/config.site");

        Command::new(src.join("configure").canonicalize().unwrap())
            .env("CONFIG_SITE", config_site.display().to_string())
            .current_dir(src)
            .run()
    }

    /// prep: runs mkmakemod.sh, creates Zle/ Builtins/ etc. subdirs in build tree
    pub(super) fn make_prep(src: &Path) {
        Command::new("make")
            .arg("-C")
            .arg("Src")
            .arg("prep")
            .current_dir(src)
            .run()
    }

    /// proto: runs makepro.awk over all .c files, generates .epro files
    pub(super) fn make_headers(src: &Path) {
        Command::new("make")
            .arg("-C")
            .arg("Src")
            .arg("headers")
            .arg("-j")
            .current_dir(src)
            .run()
    }

    trait RunCommand {
        fn run(&mut self);
    }

    impl RunCommand for Command {
        fn run(&mut self) {
            let status = self.status().expect("failed to spawn command");
            assert!(status.success(), "command failed: {:?}", self);
        }
    }
}
