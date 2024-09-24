use std::path::Path;
use std::path::PathBuf;

pub mod git;

pub use git::Repo;
pub use git::RepoTree;

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

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub struct DirEntry {
  pub path: PathBuf,
  pub is_dir: bool,
}

#[derive(Debug)]
pub struct Stat {
  pub blob_oid: git2::Oid,
  pub size: usize,
  pub is_dir: bool,
  pub is_binary: bool,
}

pub trait ReadOnlyFS {
  fn exists(&self, path: impl AsRef<Path>) -> Result<bool>;
  fn stat(&self, path: impl AsRef<Path>) -> Result<Stat>;
  fn read_dir(&self, path: impl AsRef<Path>) -> Result<Vec<DirEntry>>;
  fn read_to_vec(&self, path: impl AsRef<Path>) -> Result<Vec<u8>>;
  fn read_to_string(&self, path: impl AsRef<Path>) -> Result<String>;
}
