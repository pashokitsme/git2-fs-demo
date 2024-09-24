use core::str;
use std::error::Error;
use std::fs;
use std::os::unix::ffi::OsStrExt;
use std::path::Path;
use std::path::PathBuf;

use git2::*;

use git2_fs::git::Repo;
use git2_fs::git::RepoTree;
use git2_fs::ReadOnlyFS;
use rstest::fixture;
use rstest::rstest;

// pub struct GitFS(Repository);

// impl GitFS {
//   pub fn new(path: impl AsRef<Path>) -> Result<Self, Error> {
//     let repo = Repository::open(path)?;
//     Ok(Self(repo))
//   }

//   pub fn exists(&self, path: impl AsRef<Path>) -> bool {
//     let tree = self.0.head().unwrap().peel_to_tree().unwrap();
//     tree.get_path(path.as_ref()).is_ok()
//   }

//   pub fn read_file_from_tag(&self, tag: Option<&str>, path: impl AsRef<Path>) -> Result<String, Error> {
//     let tag = self.find_tag_by_name(tag.unwrap()).unwrap();
//     let tree = tag.target().unwrap().peel_to_tree().unwrap();
//     // let tree = self.0.head()?.peel_to_tree()?;
//     let oid = tree.get_path(path.as_ref())?.id();
//     let blob = self.0.find_blob(oid)?;
//     Ok(String::from_utf8(blob.content().to_vec()).unwrap())

//     // let index = self.0.index()?;
//     // let entry = index.get_path(path.as_ref(), 0).unwrap();
//     // let blob = self.0.find_blob(entry.id)?;
//     // let content = String::from_utf8(blob.content().to_vec()).unwrap();
//     // Ok(content)
//   }

//   fn set_head(&self, refname: &str) -> Result<(), Error> {
//     // self.0.set_head(refname)?;
//     Ok(())
//   }

//   fn find_tag_by_name(&self, tagname: &str) -> Option<Tag> {
//     let mut tag = None;
//     self
//       .0
//       .tag_foreach(|tag_oid, name| {
//         if name == tagname.as_bytes() {
//           tag = Some(tag_oid);
//           return false;
//         }
//         true
//       })
//       .unwrap();
//     tag.map(|oid| self.0.find_tag(oid).unwrap())
//   }
// }

// #[fixture]
// fn gitfs() -> GitFS {
//   GitFS::new("gramax-multilang-2").unwrap()
// }

// #[rstest]
// #[case(".doc-root.yaml", true)]
// #[case("not-exists", false)]
// fn exists(gitfs: GitFS, #[case] path: &str, #[case] expect: bool) {
//   assert_eq!(gitfs.exists(path), expect)
// }

// #[rstest]
// fn get_content(gitfs: GitFS) {
//   let content = gitfs
//     .read_file_from_tag(Some("refs/tags/tag"), ".doc-root.yaml")
//     .unwrap();
//   assert_eq!(content, "test\n");

//   let content = gitfs
//     .read_file_from_tag(Some("refs/tags/master"), ".doc-root.yaml")
//     .unwrap();

//   assert_eq!(
//     content,
//     "title: Новый каталог\nsupportedLanguages:\n  - ru\n  - en\ncode: \"\"\ndescription: null\nstyle: null\nlanguage: ru\n"
//   );
// }

// #[rstest]
// fn read_dir(gitfs: GitFS) {
//   let dir = gitfs.read_dir("en").unwrap();
//   assert_eq!(dbg!(dir).len(), 3)
// }

// #[rstest]
// fn read_file_as_dir(gitfs: GitFS) {
//   let dir = gitfs.read_dir("en/_index.md");
//   assert!(dir.is_err())
// }

// #[rstest]
// fn switch_branch(gitfs: GitFS) {
//   let content = gitfs.read_file(".doc-root.yaml").unwrap();
//   assert!(content.len() > 20);
//   gitfs.set_head("refs/heads/branch").unwrap();
//   let content = gitfs.read_file(".doc-root.yaml").unwrap();
//   assert_eq!(content, "test");
//   gitfs.set_head("refs/heads/master").unwrap();
// }

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

  let repo = git2_fs::git::Repo::open("gramax-board").unwrap();
  let start = Instant::now();
  read_dir_all(&repo.commit("6b4887b51ec0959f32521ad332db7dab8d486040".parse().unwrap()).unwrap(), PathBuf::new());
  println!("took: {:?}", start.elapsed());

  // let start = Instant::now();
  // read_dir_recursive(&PathBuf::from("gramax-board")).unwrap();
  // println!("took read dir: {:?}", start.elapsed());
}
