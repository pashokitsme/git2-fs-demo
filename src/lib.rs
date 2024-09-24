use std::path::Path;
use std::path::PathBuf;

pub mod git;

#[derive(Debug)]
pub enum Error {
  Git(git2::Error),
  Any(Box<dyn std::error::Error>),
}

impl From<git2::Error> for Error {
  fn from(value: git2::Error) -> Self {
    Self::Git(value)
  }
}

impl Error {
  fn new(message: &str) -> Self {
    Self::Any(Box::new(std::io::Error::new(std::io::ErrorKind::Other, message)))
  }
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub struct DirEntry {
  pub path: PathBuf,
  pub oid: git2::Oid,
  pub is_dir: bool,
}

pub trait ReadOnlyFS {
  fn exists(&self, path: impl AsRef<Path>) -> Result<bool>;
  fn read_dir(&self, path: impl AsRef<Path>) -> Result<Vec<DirEntry>>;
  fn read_to_vec(&self, path: impl AsRef<Path>) -> Result<Vec<u8>>;
  fn read_to_string(&self, path: impl AsRef<Path>) -> Result<String>;
}
