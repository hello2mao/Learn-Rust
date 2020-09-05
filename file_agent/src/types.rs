use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::prelude::v1::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HandleFileInfo {
  pub local: PathBuf,
  pub remote: url::Url,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum HandleFileCommand {
  Download,
  // Upload,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FileAgentRequest {
  pub cmd: HandleFileCommand,
  pub info: Vec<HandleFileInfo>,
  pub fusion_base: PathBuf,
}

impl FileAgentRequest {
  pub fn new<T: IntoIterator>(
    cmd: HandleFileCommand,
    info: T,
    fusion_base: impl AsRef<Path>,
  ) -> Self
  where
    <T as IntoIterator>::Item: Into<HandleFileInfo>,
  {
    FileAgentRequest {
      cmd,
      info: info.into_iter().map(|x| x.into()).collect(),
      fusion_base: fusion_base.as_ref().to_owned(),
    }
  }
}

impl HandleFileInfo {
  pub fn new(local: impl AsRef<Path>, remote: &url::Url) -> Self {
    HandleFileInfo {
      local: local.as_ref().to_owned(),
      remote: remote.to_owned(),
    }
  }
}
