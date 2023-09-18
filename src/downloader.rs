use std::path::{Path, PathBuf};
use tempdir::TempDir;
use tokio::process::Command;

#[derive(Debug)]
pub struct DownloadManager;

#[derive(Debug)]
pub struct TemporaryFile {
    _dir: TempDir,
    file: PathBuf,
}

impl DownloadManager {
    pub async fn download(video_url: &str) -> Result<TemporaryFile, String> {
        let tempdir = TempDir::new("download")
            .map_err(|e| format!("Failed to create a temporary directory: {e}"))?;
        let mut command = Self::generate_command(video_url, tempdir.path());

        let output = command.output().await.map_err(|e| e.to_string())?;

        if !output.status.success() {
            return Err("Exit code not zero".to_string());
        }

        let mut read = tempdir
            .path()
            .read_dir()
            .map_err(|e| format!("Failed to read temporary directory: {e}"))?;
        let file = read
            .next()
            .ok_or("Target file not found".to_string())?
            .map_err(|e| format!("Failed to stat output file: {e}"))?
            .path();

        Ok(TemporaryFile {
            _dir: tempdir,
            file,
        })
    }

    fn generate_command(video_link: &str, out_path: &Path) -> Command {
        let mut base = Command::new("yt-dlp");

        // Link must go first
        base.arg(video_link);

        // Static arguments (these should not change)
        base.args([
            "-q",
            "--no-playlist",
            "--no-continue",
            "--no-part",
            "--no-cache-dir",
            "--rm-cache-dir",
            "--no-warnings",
            "-f",
            "mp4",
        ]);

        // Dynamic arguments
        base.arg("-o");
        base.arg(out_path.join("%(id)s.%(ext)s"));

        base
    }
}

impl TemporaryFile {
    pub fn path(&self) -> &Path {
        &self.file
    }
}
