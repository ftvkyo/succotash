use async_std::fs;
use async_std::prelude::*;

pub mod img;

async fn run_impl(dir: async_std::path::PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let mut entries = fs::read_dir(dir).await?;

    while let Some(res) = entries.next().await {
        let entry = res?;

        let img = img::Img::load(entry.path())?;
        let imghash = img::ImgHash::from(img);

        println!(
            "{} has imghash of {}",
            entry.file_name().to_string_lossy(),
            imghash.hash
        );
    }

    Ok(())
}

pub async fn run(dir: async_std::path::PathBuf) {
    match run_impl(dir).await {
        Ok(_) => debug!("Done 'analyze'."),
        Err(e) => error!("Error during 'analyze': {}", e),
    }
}
