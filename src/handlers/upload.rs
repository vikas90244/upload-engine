use axum::{
    body::Bytes,
    http::{HeaderMap, StatusCode},
};
// use tokio::fs::OpenOptions;
// use tokio::io::AsyncWriteExt;

pub async fn receive_chunks(
    headers:HeaderMap,
    bytes:Bytes
) -> Result<StatusCode, (StatusCode, String)>{
    println!("{:?}", bytes.to_vec());
    println!("all good");
    Ok(StatusCode::ACCEPTED)
    }
