use std::borrow::Cow;
use std::ops::Deref;
use std::path::Path;

use crate::DirEntry;
use crate::ReadOnlyFS;
use crate::Result;
use git2::*;

pub struct Repo(pub Repository);

pub struct RepoTree<'t> {
  repo: &'t Repo,
  tree: Tree<'t>,
}

impl Deref for RepoTree<'_> {
  type Target = Repo;

  fn deref(&self) -> &Self::Target {
    self.repo
  }
}

impl Repo {
  pub fn open(path: impl AsRef<Path>) -> Result<Self> {
    Ok(Repo(Repository::open(path.as_ref())?))
  }

  pub fn head(&self) -> Result<RepoTree> {
    Ok(RepoTree { repo: self, tree: self.0.head()?.peel_to_tree()? })
  }

  pub fn commit(&self, oid: Oid) -> Result<RepoTree> {
    Ok(RepoTree { repo: self, tree: self.0.find_commit(oid)?.tree()? })
  }

  pub fn tag(&self, tagname: &str) -> Result<RepoTree> {
    let tag = self.find_tag_by_name(tagname)?;
    let tree = tag.target()?.peel_to_tree()?;
    Ok(RepoTree { repo: self, tree })
  }

  fn find_tag_by_name(&self, tagname: &str) -> Result<Tag> {
    let tagname =
      if tagname.contains("refs/tags") { Cow::Borrowed(tagname) } else { Cow::Owned(format!("refs/tags/{}", tagname)) };
    let mut tag_oid = None;

    self.0.tag_foreach(|oid, name| {
      if name == tagname.as_bytes() {
        tag_oid = Some(oid);
        return false;
      }
      true
    })?;

    Ok(
      tag_oid
        .ok_or_else(|| git2::Error::new(ErrorCode::NotFound, ErrorClass::Tag, format!("Tag {} not found", tagname)))
        .and_then(|t| self.0.find_tag(t))?,
    )
  }
}

impl ReadOnlyFS for RepoTree<'_> {
  fn exists(&self, path: impl AsRef<Path>) -> Result<bool> {
    match self.tree.get_path(path.as_ref()) {
      Ok(_) => Ok(true),
      Err(err) if err.code() == ErrorCode::NotFound => Ok(false),
      Err(err) => Err(err.into()),
    }
  }

  fn read_dir(&self, path: impl AsRef<Path>) -> Result<Vec<DirEntry>> {
    let tree_object = if path.as_ref().parent().is_none() {
      Some(Cow::Borrowed(&self.tree))
    } else {
      self
        .tree
        .get_path(path.as_ref())?
        .to_object(&self.repo.0)?
        .into_tree()
        .map(Cow::Owned)
        .ok()
    };

    let Some(tree) = tree_object else {
      return Err(git2::Error::new(ErrorCode::Invalid, ErrorClass::Tree, "tried to list directires of file").into());
    };

    let paths = tree
      .iter()
      .filter_map(|entry| {
        entry.name().map(|name| DirEntry {
          oid: entry.id(),
          path: path.as_ref().join(name),
          is_dir: entry.kind().is_some_and(|k| k == ObjectType::Tree),
        })
      })
      .collect();
    Ok(paths)
  }

  fn read_to_vec(&self, path: impl AsRef<Path>) -> Result<Vec<u8>> {
    let entry = self.tree.get_path(path.as_ref())?;
    let blob = self.0.find_blob(entry.id())?;
    Ok(blob.content().to_vec())
  }

  fn read_to_string(&self, path: impl AsRef<Path>) -> Result<String> {
    Ok(String::from_utf8_lossy(&self.read_to_vec(path)?).to_string())
  }
}
