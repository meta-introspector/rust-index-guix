mod sexpr;
#[cfg(feature = "git")]
mod git;
#[cfg(feature = "git")]
pub use git::*;

pub const GUIX_REPO_URL: &str = "https://git.savannah.gnu.org/git/guix.git";

#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct Package<S> {
    pub name: S,
    pub version: S,
}

/// Parse an .scm file and extract `(define-public … (crate-uri …))` crates from it
pub fn parse_scm<'a>(file: &'a str) -> Result<impl Iterator<Item=Package<&'a str>>, String> {
    let all = sexpr::parse(file)?;
    Ok(all.into_iter().filter_map(|s| {
        let s = s.into_list()?;
        if s.len() < 3 || s[0].as_str()? != "define-public" {
            return None;
        }
        let package = s.into_iter().nth(2)?.into_list()?;
        if package.get(0)?.as_str()? != "package" {
            return None;
        }
        let name = package.iter().skip(1).find_map(|arg| {
            let origin = arg.if_named("source")?.get(0)?.if_named("origin")?;
            let uri = origin.iter().find_map(|arg| arg.if_named("uri")?.get(0)?.if_named("crate-uri"))?;
            uri.get(0)?.as_str()
        })?;
        let version = package.into_iter().skip(1).find_map(|arg| {
            arg.if_named("version")?.get(0)?.as_str()
        })?;
        Some(Package { name, version })
    }))
}
