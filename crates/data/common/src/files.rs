use crate::{DecodedBody, Error, Result};
use std::{
    path::{Path, PathBuf},
    sync::atomic::{AtomicU64, Ordering},
};
use tokio::io::AsyncWriteExt;
static NEXT: AtomicU64 = AtomicU64::new(0);
struct Partial(PathBuf);
impl Drop for Partial {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.0);
    }
}
/// The destination must not exist. A sibling temporary file is removed on error,
/// cancellation or drop. A hard link publishes the complete file without replacing
/// an existing destination, including one created concurrently.
pub async fn download_to(mut body: DecodedBody, path: impl AsRef<Path>) -> Result<u64> {
    let path = path.as_ref();
    let parent = path
        .parent()
        .filter(|p| !p.as_os_str().is_empty())
        .unwrap_or(Path::new("."));
    let temp = parent.join(format!(
        ".aux-download-{}-{}.part",
        std::process::id(),
        NEXT.fetch_add(1, Ordering::Relaxed)
    ));
    // Create and install the guard without an await boundary. An async open may
    // complete after cancellation, leaving a file before its guard exists.
    let file = std::fs::OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(&temp)?;
    let partial = Partial(temp);
    let mut file = tokio::fs::File::from_std(file);
    let mut size = 0u64;
    while let Some(bytes) = body.next().await {
        let bytes = bytes?;
        file.write_all(&bytes).await?;
        size = size
            .checked_add(bytes.len() as u64)
            .ok_or(Error::Decode("download size overflow"))?;
    }
    file.flush().await?;
    file.sync_all().await?;
    drop(file);
    // Commit without an await boundary that could finish after cancellation.
    std::fs::hard_link(&partial.0, path)?;
    Ok(size)
}
