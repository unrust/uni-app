use std;
use std::io::ErrorKind;
use std::io::Read;
use std::str;

/// the root filesystem API
pub struct FileSystem {}
/// synchronous (native) / asynchronous (web) file API
pub enum File {
    Native(std::fs::File),
}

pub type IoError = std::io::Error;
pub type IoErrorKind = std::io::ErrorKind;

impl FileSystem {
    /// open a file.
    /// For a file to be accessible from both native and web build, it should be placed
    /// in a www/ directory in your project's root directory, for example www/config.json.
    /// You can then open this file with `FileSystem::open("config.json")`.
    /// When running your native project, use `cd www && cargo run`.
    /// When deploying on the web, the file should simply be in the same directory as index.html, as config.json.
    /// Note that on wasm target, this works with any URL, you can do :
    /// `FileSystem::open('https://raw.githubusercontent.com/unrust/uni-app/master/www/test.txt')`
    /// To be able to load data from an URL in native mode, you have to enable the http feature.
    pub fn open(s: &str) -> Result<File, IoError> {
        if s.starts_with("http") {
            Err(std::io::Error::new(
                ErrorKind::Other,
                format!(
                    "error : could not read {} : http feature not enabled in uni-app::FileSystem",
                    s
                ),
            ))
        } else {
            Ok(File::Native(std::fs::File::open(s)?))
        }
    }
}

impl File {
    /// Once the file has been loaded (see [`File::is_ready`]), returns the file content as `Vec<u8>`
    pub fn read_binary(&mut self) -> Result<Vec<u8>, IoError> {
        let mut buf = Vec::new();
        match self {
            File::Native(f) => {
                f.read_to_end(&mut buf)?;
            }
        }
        Ok(buf)
    }
    /// Once the file has been loaded (see [`File::is_ready`]), returns the file content as a String
    pub fn read_text(&mut self) -> Result<String, IoError> {
        let mut data = String::new();
        match self {
            File::Native(f) => match f.read_to_string(&mut data) {
                Ok(_) => Ok(data),
                Err(e) => Err(std::io::Error::new(ErrorKind::Other, e)),
            },
        }
    }
    /// return true if the file has been loaded
    /// On native target, files are loaded synchronously.
    /// As soon as [`FileSystem::open`] returns, the file is ready.
    /// [`File::read_binary`] and [`File::read_text`] can be called immediately.
    /// On web target, files are loaded asynchronously.
    /// You have to poll [`File::is_ready`] until it returns true.
    /// Only then you can call [`File::read_binary`] or [`File::read_text`].
    pub fn is_ready(&mut self) -> bool {
        true
    }
}
