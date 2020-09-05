use crate::types::{FileAgentRequest, HandleFileCommand, HandleFileInfo};
use std::path::{Path, PathBuf};
use url::Url;
use tokio::io::AsyncWriteExt;
use futures::future::join_all;
use futures::TryFutureExt;

async fn download_remote_input_to_file(
  presigned_url: Url,
  dest: impl AsRef<Path>,
) -> anyhow::Result<()> {
  let mut download = reqwest::get(presigned_url.as_str())
    .await?
    .error_for_status()?;

  let mut out_file = tokio::fs::File::create(dest).await?;
  
  while let Some(chunk) = download.chunk().await? {
    out_file.write_all(&chunk).await?;
  }

  out_file.flush().await?;
  
  Ok(())
}

async fn handle_download(
  info: HandleFileInfo,
  fusion_base: impl AsRef<Path>,
) -> anyhow::Result<()> {
  debug!("[handle_download]");
  anyhow::ensure!(
    !info.local.exists(),
    "[Download] Dest local file: {:?} already exist.",
    info.local
  );

  let dst = info.local;
  let remote = info.remote;

  match remote.scheme() {
    "https" | "http" => {
      download_remote_input_to_file(remote, dst).await?;  
    }
    _ => anyhow::bail!("Scheme not supported"),
  }

  Ok(())
}

fn handle_file_request(bytes: &[u8]) -> anyhow::Result<()> {
  debug!("[handle_file_request]");
  let req: FileAgentRequest = serde_json::from_slice(bytes)?;
  let results = tokio::runtime::Builder::new()
    .threaded_scheduler()
    .enable_all()
    .build()?
    .block_on(async {
      let fusion_base = req.fusion_base.clone();
      match req.cmd {
        HandleFileCommand::Download => {
          let features: Vec<_> = req
            .info
            .into_iter()
            .map(|info| {
              let fusion_base = fusion_base.clone();
              tokio::spawn(async {
                handle_download(info, fusion_base).await
              })
            })
            .collect();
            join_all(features).await
        }
      }
    });
  
  let (task_results, errs): (Vec<_>, Vec<_>) = results.into_iter().partition(Result::is_ok);

  debug!("{:?}, errs: {:?}", task_results, errs);
  if !errs.is_empty() {
    anyhow::bail!("spawned task join err")
  }
  anyhow::ensure!(
    task_results.into_iter().all(|x| x.unwrap().is_ok()),
    "Some handle file task is failed"
  );
    
  Ok(())
}

#[no_mangle]
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub extern "C" fn ocall_handle_file_request(in_buf: *const u8, in_len: u32) -> u32 {
  let input_buf: &[u8] = unsafe { std::slice::from_raw_parts(in_buf, in_len as usize) };
  match handle_file_request(input_buf) {
    Ok(_) => 0,
    Err(_) => 1,
  }
}

#[cfg(test)]
mod tests {

  use super::*;
  use url::Url;
  use std::path::PathBuf;
  use std::io::Write;

  #[test]
  fn test_get_single_file() {
    let s = "http://idea.medeming.com/jets/images/jihuoma.zip";
    let url = Url::parse(s).unwrap();
    let dest = PathBuf::from("/tmp/jihuoma_test.zip");

    let info = HandleFileInfo::new(&dest, &url);
    let req = FileAgentRequest::new(HandleFileCommand::Download, vec![info], "");

    let bytes = serde_json::to_vec(&req).unwrap();
    handle_file_request(&bytes).unwrap();

    std::fs::remove_file(&dest).unwrap();
  }
}
