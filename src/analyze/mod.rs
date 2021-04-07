//! Subcommand 'analyze' lives here.
//!
//! This module contains the 'analyze' subcommand.
//! Analyze allows finding similar images in a directoy.

use async_std::fs;
use async_std::prelude::*;

pub mod img;

/// Run the analysis on the given path.
///
/// # Arguments
///
/// * `dir` - Where to run the analysis.
async fn try_run(dir: async_std::path::PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    debug!("Loading dir '{}' entries...", dir.to_string_lossy());
    let mut entries = fs::read_dir(&dir).await?;
    debug!("Loaded dir '{}' entries", dir.to_string_lossy());

    while let Some(res) = entries.next().await {
        let entry = res?;

        debug!("Synchronously opening image '{}'", entry.file_name().to_string_lossy());
        let img = img::Img::load(entry.path())?;
        debug!("Getting the imghash of image '{}'", entry.file_name().to_string_lossy());
        let imghash = img::ImgHash::from(img);

        info!(
            "img '{}' has imghash of {}",
            entry.file_name().to_string_lossy(),
            imghash.hash
        );
    }

    Ok(())
}

/// Run the analysis on the given path, do not propagate errors.
///
/// You can think of it as of `main` of the `analyze` subcommand.
///
/// # Examples
///
/// ```no_run
/// # use libsuccotash::analyze;
/// analyze::run("/home/user/Pictures".into());
/// ```
pub async fn run(dir: async_std::path::PathBuf) {
    match try_run(dir).await {
        Ok(_) => debug!("Done 'analyze'"),
        Err(e) => error!("Error during 'analyze': {}", e),
    }
}
