use axum::{
    body::Bytes,
    http::{HeaderMap, HeaderValue, StatusCode},
};
use std::path::PathBuf;

use tokio::fs::OpenOptions;
use tokio::io::AsyncWriteExt;
use base64::{engine::general_purpose::STANDARD, Engine as _};
// use tokio::fs::OpenOptions;
// use tokio::io::AsyncWriteExt;

pub async fn receive_chunks(
    req_headers: HeaderMap,
    bytes: Bytes,
) -> Result<(StatusCode, HeaderMap), (StatusCode, String)> {



     // Let's get upload metadata
    let filename = match req_headers.get("Upload-Metadata")
                            .and_then(|v| v.to_str().ok())
                            .and_then(|v| {
                            let exctracted:Vec<&str>= v.split(" ").collect();
                            let decoded = STANDARD.decode(exctracted[1]).ok()?;
                            let decoded_str = String::from_utf8(decoded).ok()?;
                            println!("decoded metadata filename is: {:?}", decoded_str);
                            Some(decoded_str)
                            }){
                            Some(v)=> v,
                            None => "invalid".to_string(),
                            };


    // Create file if doesn't exist
    // Write to file if it exists
    let dir = upload_dir();
    let file_path = dir.join(filename);
    tokio::fs::create_dir_all(&dir).await
                         .map_err(|e|  (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    let mut file = OpenOptions::new()
                        .create(true)
                        .append(true)
                        .open(&file_path).await
                        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    // Write to file 
    file.write_all(&bytes).await    
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;


   
    // offset calculation from file metadata
    let offset = file
        .metadata().await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .len();

    println!("offset in rust is: {}", offset);
    let mut headers = HeaderMap::new();
    headers.insert(
        "Upload-Offset",
        HeaderValue::from_str(&offset.to_string()).map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?,
    );

    Ok((StatusCode::ACCEPTED, headers))
    }


fn upload_dir() -> PathBuf {
    PathBuf::from("/home/vikas/semicolon/uploads")
}