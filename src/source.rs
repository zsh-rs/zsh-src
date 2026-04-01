use crate::download::ZshSourceDownloader;

pub struct ZshSource {
    pub version: String,
    pub source: std::path::PathBuf,
}

impl From<ZshSourceDownloader> for ZshSource {
    fn from(downloader: ZshSourceDownloader) -> Self {
        let version = downloader.version.unwrap();
        let source = downloader.source.unwrap();

        Self { version, source }
    }
}
