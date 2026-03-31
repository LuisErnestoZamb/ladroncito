use reqwest;
use std::fs;

pub async fn pull_file(url: &str, job_id: String) -> Result<String, Box<dyn std::error::Error>> {
    let file_path = format!("/tmp/analysis_{}.csv", job_id);

    let response_bytes = reqwest::get(url).await?.bytes().await?;

    fs::write(&file_path, response_bytes)?;

    Ok(file_path)
}
