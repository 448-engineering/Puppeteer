use crate::{PuppeteerError, PuppeteerResult, StaticCowStr};
use base64ct::{Base64, Encoding};
use bytes::{BufMut, BytesMut};
use camino::{Utf8Path, Utf8PathBuf};
use file_format::FileFormat;
use smol::{fs::File, io::AsyncReadExt};
use std::borrow::Cow;

/// The buffer capacity of a bufreader
pub const BUFFER_CAPACITY: usize = 1024 * 64; //16KiB
/// The default resource size set to avoid high memory usage and data
pub const DEFAULT_RESOURCE_SIZE: usize = 1024 * 1024; //1MiB

/// Methods to detect file type and convert to encoding formats like base64
pub trait FileProperties {
    /// The name of the resource
    fn name(&self) -> Cow<str>;

    /// The [FileFormat] of the resource
    fn format(&self) -> FileFormat;

    /// The content bytes
    fn bytes(&self) -> &BytesMut;

    /// Base64 encoding for html
    fn base64(&self) -> Cow<str>;
}

/// An asset to be used in the app
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct AssetFile {
    /// The name of the asset
    pub name: StaticCowStr,
    /// The bytes contained in the asset
    pub bytes: BytesMut,
}

impl FileProperties for AssetFile {
    fn name(&self) -> StaticCowStr {
        self.name.clone()
    }

    fn format(&self) -> FileFormat {
        FileFormat::from_bytes(&self.bytes)
    }

    fn bytes(&self) -> &BytesMut {
        &self.bytes
    }

    fn base64(&self) -> StaticCowStr {
        let media_type = self.format().media_type().to_owned();

        Cow::Borrowed("data:")
            + Cow::Owned(media_type)
            + ";base64,"
            + Cow::Owned(Base64::encode_string(&self.bytes))
    }
}

/// Default resource file size is 1MiB
#[derive(Debug)]
pub struct AssetFileLoader<'p> {
    dir: &'p Utf8Path,
    file_name: &'p Utf8Path,
    max_resource_size: usize,
    cargo_dir: bool,
}

impl<'p> AssetFileLoader<'p> {
    /// Instantiate a new [ResourceFile]
    pub fn new(file_name: &'p str) -> Self {
        AssetFileLoader {
            dir: Utf8Path::new(env!("CARGO_MANIFEST_DIR")),
            file_name: Utf8Path::new(file_name),
            max_resource_size: DEFAULT_RESOURCE_SIZE,
            cargo_dir: true,
        }
    }

    /// Ensure the path is loaded from Cargo Workspace dir
    pub fn path_from_cargo_dir(mut self, root_cargo_dir: bool) -> Self {
        self.cargo_dir = root_cargo_dir;

        self
    }

    /// Add the directory relative to `CARGO_MANIFEST_DIR`
    pub fn add_dir(mut self, dir: &'p str) -> Self {
        self.dir = Utf8Path::new(dir);

        self
    }

    /// Change maximum size of the resource from the default 1MiB
    pub fn max_resource_size(mut self, size: usize) -> Self {
        self.max_resource_size = size;

        self
    }

    /// Load the resource
    pub async fn load(self) -> PuppeteerResult<AssetFile> {
        let mut path = Utf8PathBuf::new();

        if self.cargo_dir {
            path.push(env!("CARGO_MANIFEST_DIR"));
        };
        path.push(self.dir);
        path.push(self.file_name);

        let mut file = File::open(path).await?;
        let mut container = BytesMut::with_capacity(self.max_resource_size);
        let mut bytes_read: usize;
        let mut buffer = [0; BUFFER_CAPACITY];

        loop {
            if container.len() > self.max_resource_size {
                return Err(PuppeteerError::MaxResourceLengthExceeded);
            }

            bytes_read = file.read(&mut buffer).await?;

            if bytes_read == 0 {
                break;
            }

            if buffer[..bytes_read].len() < BUFFER_CAPACITY {
                container.put(&buffer[..bytes_read][..]);
                break;
            }

            container.put(&buffer[..bytes_read][..]);
        }

        let file_name = self.file_name.to_path_buf();

        Ok(AssetFile {
            name: Cow::Owned(file_name.to_string()),
            bytes: container,
        })
    }
}

#[cfg(test)]
mod asset_checks {
    use super::*;

    #[test]
    fn correctness() {
        smol::block_on(async {
            {
                let resource = AssetFileLoader::new("centauri.woff2")
                    .add_dir("examples/assets/fonts")
                    .load()
                    .await
                    .unwrap();

                assert_eq!(&FileFormat::WebOpenFontFormat2, &resource.format());
                assert_eq!(&file_format::Kind::Font, &resource.format().kind());
                assert_eq!(&"font/woff2", &resource.format().media_type());
                assert_eq!(&Some("WOFF2",), &resource.format().short_name());
                assert_eq!("centauri.woff2", &resource.name());
            }

            {
                let resource = AssetFileLoader::new("frow.min.css")
                    .add_dir("examples/assets")
                    .load()
                    .await
                    .unwrap();

                assert_eq!(&FileFormat::PlainText, &resource.format());
                assert_eq!(&file_format::Kind::Text, &resource.format().kind());
                assert_eq!(&"text/plain", &resource.format().media_type());
                assert_eq!(&Some("TXT",), &resource.format().short_name());
                assert_eq!("frow.min.css", &resource.name());
            }
        })
    }
}
