pub async fn download_image(url: &str, filename: &str, directory: &str) -> Result<PathBuf> {
    let response = reqwest::get(url).await?;
    if !response.status().is_success() {
        return Err(anyhow!("Unable to download image"));
    }
    let image_data = response.bytes().await?;
    let filepath = Path::new(directory).join(filename);
    fs::write(filepath.clone(), image_data)?;
    Ok(filepath)
}
