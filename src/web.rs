use std::path::Path;
use follow_redirects::ClientExt;
use hyper::Client;
use hyper::body::HttpBody;
use hyper::header::CONTENT_LENGTH;
use hyper_tls::HttpsConnector;
use tokio::fs::{File, self};
use tokio::io::{AsyncWriteExt, BufWriter};
use crate::config::Configuration;
use crate::util::progress::ProgressBar;

pub async fn get_body(url: &String) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let https = HttpsConnector::new();
    let client = Client::builder()
        .build::<_, hyper::Body>(https);
    
    let uri = url.parse()?;

    let mut response = client.follow_redirects().get(uri).await?;

    let mut buffer = vec![];

    while let Some(chunk) = response.body_mut().data().await {
        buffer.append(&mut chunk?.to_vec());
    }

    Ok(String::from_utf8(buffer).unwrap())
}

/// Downloads the jar from the configuration URL
pub async fn download_server(config: &Configuration, target: &String) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let uri_ref = &config.server;
    let server = if uri_ref.starts_with("https://paper-fetcher.netlify.app/paper") {
        get_body(uri_ref).await?
    } else {
        uri_ref.clone()
    };

    download_progress(&server, target).await?;
    Ok(())
}

/// Downloads the file from the url and saves it to the target
pub async fn download(url: &String, target: &String) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let mut buffer = BufWriter::new(File::create(&target).await?);

    let https = HttpsConnector::new();
    let client = Client::builder()
        .build::<_, hyper::Body>(https);
    
    let uri = url.parse()?;

    let mut response = client.follow_redirects().get(uri).await?;

    while let Some(chunk) = response.body_mut().data().await {
        buffer.write_all(&chunk?).await?;
    }

    buffer.flush().await?;

    Ok(())
}

pub async fn download_progress(url: &String, target: &String) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let temp_dir = Path::new(".temp");
    if !temp_dir.exists() {
        fs::create_dir(temp_dir).await?;
    }

    let target_file = Path::new(target);
    let path_buf = temp_dir.join(target_file.file_name().unwrap());
    let temp_target_file = Path::new(path_buf.as_os_str());

    let mut buffer = BufWriter::new(File::create(temp_target_file).await?);

    let https = HttpsConnector::new();
    let client = Client::builder()
        .build::<_, hyper::Body>(https);

    let clip = url.parse()?;

    let mut response = client.follow_redirects().get(clip).await?;
    let total = response.headers()[CONTENT_LENGTH].to_str().unwrap().parse::<f32>()?;

    let mut bar = ProgressBar::new(total as i32);
    let mut read = 0;

    while let Some(chunk) = response.body_mut().data().await {
        let chunk = &chunk?;
        read += chunk.len();
        buffer.write_all(chunk).await?;
        bar.set(read as i32);
        bar.print();
    }

    buffer.flush().await?;

    fs::copy(temp_target_file, target_file).await?;

    fs::remove_dir_all(temp_dir).await?;

    // Exit Carriage Return
    bar.clear_text();
    Ok(())
}