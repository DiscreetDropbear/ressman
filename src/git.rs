use git2::{Cred, Error, RemoteCallbacks, Repository};
use std::env;
use std::path::Path;

/// if the repo to be cloned uses ssh calls clone_ssh otherwise
/// does the clone itself
pub fn clone_repo(url: &str, dest_path: &Path) -> Result<Repository, ()>{
  if url.starts_with("https"){
    Ok(Repository::clone(url, dest_path).unwrap())  
  }
  else{
    Ok(clone_ssh(url, dest_path)?)
  }
}

pub fn clone_ssh(url: &str, dest_path: &Path) -> Result<Repository, ()>{

  // Prepare callbacks.
  let mut callbacks = RemoteCallbacks::new();
  callbacks.credentials(|_url, username_from_url, _allowed_types| {
    Cred::ssh_key(
      username_from_url.unwrap(),
      None,
      std::path::Path::new(&format!("{}/.ssh/id_rsa", env::var("HOME").unwrap())),
      None,
    )
  });

  let mut fo = git2::FetchOptions::new();
  fo.remote_callbacks(callbacks);

  let mut builder = git2::build::RepoBuilder::new();
  builder.fetch_options(fo);

  // TODO: handle the errors gracefully and recover when possible
  let repo = builder.clone(
    url,
    dest_path
  ).unwrap(); 

  Ok(repo)
}
