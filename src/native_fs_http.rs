use std;
use std::io::ErrorKind;
use std::io::Read;
use std::str;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::TryRecvError;
use std::thread::JoinHandle;

/// the root filesystem API
pub struct FileSystem {}
/// synchronous (native) / asynchronous (web) file API
pub enum File {
    Native(std::fs::File),
    Data(Option<Vec<u8>>),
    Pending(Receiver<Result<Option<Vec<u8>>, IoError>>, JoinHandle<()>),
    Error(IoError),
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
            if cfg!(feature = "http") {
                let (tx, rx) = std::sync::mpsc::channel();
                let url = s.to_owned();
                let handle = std::thread::spawn(move || match reqwest::blocking::get(url) {
                    Err(e) => {
                        tx.send(Err(std::io::Error::new(ErrorKind::Other, e)))
                            .unwrap();
                    }
                    Ok(response) => {
                        let bytes = response.bytes();
                        match bytes {
                            Err(e) => {
                                tx.send(Err(std::io::Error::new(ErrorKind::Other, e)))
                                    .unwrap();
                            }
                            Ok(bytes) => {
                                tx.send(Ok(Some(bytes.to_vec()))).unwrap();
                            }
                        }
                    }
                });
                return Ok(File::Pending(rx, handle));
            } else {
                Err(std::io::Error::new(
                    ErrorKind::Other,
                    format!(
                    "error : could not read {} : http feature not enabled in uni-app::FileSystem",
                    s
                ),
                ))
            }
        } else {
            Ok(File::Native(std::fs::File::open(s)?))
        }
    }
}

impl File {
    /// Once the file has been loaded (see [`File::is_ready`]), returns the file content as `Vec<u8>`
    pub fn read_binary(&mut self) -> Result<Vec<u8>, IoError> {
        match self {
            File::Native(f) => {
                let mut buf = Vec::new();
                f.read_to_end(&mut buf)?;
                Ok(buf)
            }
            File::Data(data) => Ok(data.take().unwrap()),
            File::Error(e) => Err(std::io::Error::new(e.kind(),e.to_string())),
            File::Pending(_,_) => Err(std::io::Error::new(ErrorKind::Other, "error: calling read_binary on a pending File. Check is_ready() before calling read_binary")),
        }
    }
    /// Once the file has been loaded (see [`File::is_ready`]), returns the file content as a String
    pub fn read_text(&mut self) -> Result<String, IoError> {
        let mut data = String::new();
        match self {
            File::Native(f) => match f.read_to_string(&mut data) {
                Ok(_) => Ok(data),
                Err(e) => Err(std::io::Error::new(ErrorKind::Other, e)),
            },
            File::Data(data) => match String::from_utf8(data.take().unwrap()) {
                Ok(s) => Ok(s),
                Err(e) =>  Err(std::io::Error::new(ErrorKind::Other, e))
            },
            File::Error(e) => Err(std::io::Error::new(e.kind(),e.to_string())),
            File::Pending(_,_) => Err(std::io::Error::new(ErrorKind::Other, "error: calling read_text on a pending File. Check is_ready() before calling read_text")),
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
        match self {
            File::Native(_) => true,
            File::Pending(rx, _) => match rx.try_recv() {
                Ok(result) => match result {
                    Ok(data) => {
                        *self = File::Data(data);
                        return true;
                    }
                    Err(e) => {
                        *self = File::Error(e);
                        return true;
                    }
                },
                Err(TryRecvError::Empty) => {
                    return false;
                }
                Err(e) => {
                    *self = File::Error(std::io::Error::new(ErrorKind::Other, e));
                    return true;
                }
            },
            File::Error(_) => true,
            File::Data(_) => true,
        }
    }
}
