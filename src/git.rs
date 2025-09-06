use std::path::PathBuf;
use gix::bstr::ByteSlice;
use gix::remote::fetch::Shallow;
use std::path::Path;
use std::sync::atomic::AtomicBool;
use gix::clone::PrepareFetch;
use gix::{create, open, ThreadSafeRepository};
use crate::Package;
use crate::GUIX_REPO_URL;

pub type Error = Box<dyn std::error::Error + Send + Sync>;

/// Reads Guix's repo
pub struct Index {
    repo: ThreadSafeRepository,
    path: PathBuf,
}

impl Index {
    /// Specify path where to place repo checkout (e.g. `"/var/tmp"`)
    ///
    /// This will clone the repo on first use. It may take a while!
    ///
    /// <div class="warning"><a href="https://github.com/Byron/gitoxide/issues/1026">
    /// This call may deadlock forever due to gix-lock.
    /// </a></div>
    #[inline]
    pub fn new(local_temp_dir: impl AsRef<Path>) -> Result<Self, Error> {
        let path = local_temp_dir.as_ref().join("git.savannah.gnu.org-git-guix.git");
        if !path.exists() {
            let stop = AtomicBool::new(false);
            Self::do_fetch(&path, &stop)?;
        }
        let repo = ThreadSafeRepository::open_opts(&path, open::Options::isolated())?;

        Ok(Self {
            repo, path,
        })
    }

    #[inline(never)]
    #[cold]
    fn do_fetch(path: &Path, stop: &AtomicBool) -> Result<(), Error> {
        let mut pre_fetch = PrepareFetch::new(GUIX_REPO_URL, path, create::Kind::Bare, create::Options::default(), open::Options::isolated())?
            .with_shallow(Shallow::DepthAtRemote(1.try_into().unwrap()));
        pre_fetch.fetch_only(gix::progress::Discard, stop)?;
        Ok(())
    }

    /// Update the repository
    ///
    /// You can flip `stop_if_true` to abort the fetch.
    ///
    /// <div class="warning"><a href="https://github.com/Byron/gitoxide/issues/1026">
    /// This call may deadlock forever due to gix-lock.
    /// </a></div>
    #[inline]
    pub fn update(&self, stop_if_true: Option<&AtomicBool>) -> Result<(), Error> {
        Self::do_fetch(&self.path, stop_if_true.unwrap_or(&AtomicBool::new(false)))
    }

    /// Parse all crates to be packaged in Guix
    #[inline(never)]
    pub fn list_all(&self) -> Result<Vec<(String, Vec<Package<String>>)>, Error> {
        let repo = self.repo.to_thread_local();
        let head = repo.head_commit()?;
        let root = head.tree()?;
        let packages_dir = root.lookup_entry_by_path("gnu/packages")?.ok_or("missing gnu/packages")?.object()?.peel_to_tree()?;

        packages_dir.iter().filter_map(|entry| {
            let entry = match entry {
                Ok(entry) => entry,
                Err(e) => return Some(Err(Error::from(e))),
            };
            let name = entry.filename().to_str().ok()?.strip_suffix(".scm")?.to_string();
            Some(Ok((name, entry)))
        }).map(|res| {
            let (name, entry) = res?;
            let blob = entry.object()?.peel_to_kind(gix::objs::Kind::Blob)?;
            let scm = String::from_utf8_lossy(&blob.data);
            if scm.trim_start().is_empty() {
                return Ok((name, vec![]));
            }
            let packages = crate::parse_scm(&scm)?.map(|p| Package {
                name: p.name.to_owned(),
                version: p.version.to_owned(),
            }).collect();
            Ok((name, packages))
        })
        .filter(|r| r.as_ref().map_or(true, |(_, pkgs)| !pkgs.is_empty()))
        .collect()
    }
}
