use std::fs;
use std::io;
use std::path::PathBuf;

use crate::cache::CacheDir;
use crate::headers::ZshSource;


impl CacheDir {
    pub(crate) fn download<'a>(self, version: &'a str) -> io::Result<ZshDownload<'a>> {
        let name = format!("zsh-{}.tar.xz", version);
        let tarball = self.root.join(&name);

        if tarball.exists() {
            return Ok(ZshDownload::new(self, tarball, version));
        }

        let url = format!("https://www.zsh.org/pub/{}", name);

        let mut res = ureq::get(&url).call().map_err(|e| e.into_io())?;

        let mut out = fs::File::create(&tarball)?;

        io::copy(&mut res.body_mut().as_reader(), &mut out)?;

        Ok(ZshDownload::new(self, tarball, version))
    }
}

pub(crate) struct ZshDownload<'a> {
    root: PathBuf,
    tarball: PathBuf,
    version: &'a str,
}

impl<'a> ZshDownload<'a> {
    pub(crate) fn new(downloader: CacheDir, tarball: PathBuf, version: &'a str) -> Self {
        Self {
            root: downloader.root,
            tarball,
            version,
        }
    }

    pub(crate) fn extract(&self) -> io::Result<ZshSource> {
        let out_dir = self.root.join(format!("zsh-{}", self.version));

        if out_dir.exists() {
            return Ok(ZshSource::new(self.version, out_dir));
        }

        let file = fs::File::open(&self.tarball)?;
        let decompressor = xz2::read::XzDecoder::new(file);
        tar::Archive::new(decompressor).unpack(&out_dir)?;

        Ok(ZshSource::new(self.version, out_dir))
    }
}
