
use std::fs;
use std::path::Path;
use std::path::PathBuf;

use git2_fs::git::RepoTree;
use git2_fs::ReadOnlyFS;

fn read_dir_all(repo: &RepoTree, path: PathBuf) {
  let dir = repo.read_dir(path).unwrap();
  for entry in dir {
    if entry.is_dir {
      read_dir_all(repo, entry.path);
    } else {
      _ = repo.read_to_vec(entry.path).unwrap();
    }
  }
}

fn list_dir_all(repo: &RepoTree, path: PathBuf, depth: usize) {
  let dir = repo.read_dir(&path).unwrap();
  for entry in dir {
    if entry.is_dir {
      println!("{:indent$}{name}", "", indent = depth * 4, name = entry.path.file_name().unwrap().to_string_lossy());
      list_dir_all(repo, entry.path, depth + 1);
    } else {
      println!("{:indent$}{name}", "", indent = depth * 4, name = entry.path.file_name().unwrap().to_string_lossy());
    }
  }
}

use std::io;

fn read_dir_recursive(path: &Path) -> io::Result<()> {
  if path.is_dir() {
    for entry in fs::read_dir(path)? {
      let entry = entry?;
      let path = entry.path();
      if path.is_dir() {
        read_dir_recursive(&path)?;
      } else {
        let _ = fs::read(&path)?;
      }
    }
  }
  Ok(())
}

fn main() {
  use std::time::Instant;

  let path = std::env::args().nth(1).expect("no path provided");

  let repo = git2_fs::git::Repo::open(path).unwrap();
  let start = Instant::now();
  list_dir_all(&repo.head().unwrap(), PathBuf::new(), 0);
  eprintln!("list dir time: {:?}", start.elapsed());
  // read_dir_recursive(&PathBuf::from("gramax-board")).unwrap();
}
