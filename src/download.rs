use std::fs;
use std::io;
use std::ops::Not;
use std::path::PathBuf;

use directories::ProjectDirs;
use file_guard::{FileGuard, Lock};
use ureq;
use xz2;
use tar;

use crate::commands;

const APP_NAME: &str = "zsh-src";
const ORG_NAME: &str = "zsh-rs";
const QUALIFIER: &str = "dev";

pub struct ZshSourceDownloader {
    root: PathBuf,
    pub(super) version: Option<String>,
    tarball: Option<PathBuf>,
    pub(super) source: Option<PathBuf>,
    _lock: Option<FileGuard<Box<fs::File>>>,
}

impl ZshSourceDownloader {
    pub(super) fn new() -> Self {
        let proj = ProjectDirs::from(QUALIFIER, ORG_NAME, APP_NAME).expect("no cache dir");

        let dir = proj.cache_dir();

        println!("[zsh-src] cache dir: {}", dir.display());

        std::fs::create_dir_all(dir).unwrap();
        ZshSourceDownloader {
            root: dir.to_path_buf(),
            version: None,
            tarball: None,
            _lock: None,
            source: None,
        }
    }

    pub(super) fn lock_version(mut self, version: String) -> io::Result<Self> {
        let lock_file = self.root.join(format!("zsh-{}.lock", version));
        let file = Box::new(
            fs::OpenOptions::new()
                .read(true)
                .write(true)
                .create(true)
                .open(lock_file)?,
        );

        self.version = Some(version);
        self._lock = Some(file_guard::lock(file, Lock::Exclusive, 0, 1)?);

        Ok(self)
    }

    pub(super) fn download<'a>(mut self) -> io::Result<Self> {
        let version = self
            .version
            .as_ref()
            .expect("must lock version before downloading");

        let name = format!("zsh-{}.tar.xz", version);
        self.tarball = Some(self.root.join(&name));
        let tarball = self.tarball.as_ref().unwrap();

        if tarball.exists() {
            return Ok(self);
        }

        println!("[zsh-src] downloading: {name}");

        let url = format!("https://www.zsh.org/pub/{}", name);
        let mut res = ureq::get(&url).call().map_err(|e| e.into_io())?;
        let mut out = fs::File::create(&tarball)?;
        io::copy(&mut res.body_mut().as_reader(), &mut out)?;

        Ok(self)
    }

    pub(super) fn extract(mut self) -> io::Result<Self> {
        let version = self
            .version
            .as_ref()
            .expect("must lock version before downloading");
        let tarball = self
            .tarball
            .as_ref()
            .expect("must be downloaded before extracting");

        self.source = Some(self.root.join(format!("zsh-{}", version)));
        let out_dir = self.source.as_ref().unwrap();

        if out_dir.exists() {
            return Ok(self);
        }

        println!("[zsh-src] extracting: {} into {}", tarball.display(), out_dir.display());

        let file = fs::File::open(tarball)?;
        let decompressor = xz2::read::XzDecoder::new(file);
        tar::Archive::new(decompressor).unpack(&self.root)?;

        Ok(self)
    }

    pub(super) fn ensure_headers(self) -> Self {
        let source = self
            .source
            .as_ref()
            .expect("must be extracted before ensuring headers");
        let complete_marker = source.join(".complete");

        if complete_marker.exists() {
            return self;
        }

        println!("[zsh-src] ensuring headers for: {}", source.display());

        source
            .join("./configure")
            .exists()
            .not()
            .then(|| commands::autoreconf(&source));

        commands::configure(&source);
        commands::make_prep(&source);
        commands::make_headers(&source);

        std::fs::write(&complete_marker, b"ok").unwrap();

        self
    }
}
