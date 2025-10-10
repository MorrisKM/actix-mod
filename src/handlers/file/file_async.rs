use actix_multipart::Multipart;
use actix_web::{post, web, HttpResponse, Responder};
use futures_util::TryStreamExt as _;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use std::path::PathBuf;

#[post("/v3/upload")]
pub async fn uploadv3(mut payload: Multipart) -> impl Responder {
    // Get current directory and create an images directory path
    let current_dir = std::env::current_dir().expect("failed to read current directory");
    let img_dir = current_dir.join("images");

    // Optionally create the images directory if it doesn't exist
    if !img_dir.exists() {
        if let Err(e) = tokio::fs::create_dir_all(&img_dir).await {
            return HttpResponse::InternalServerError()
                .content_type("text/plain")
                .body(format!("Failed to create directory: {}", e));
        }
    }

    // Process multipart fields (files)
    while let Ok(Some(mut field)) = payload.try_next().await {
        let content_disposition = match field.content_disposition() {
            Some(cd) => cd,
            None => continue, // Skip fields without content disposition
        };

        // Use filename provided or a default
        let filename = content_disposition.get_filename().unwrap_or("sample.png");

        // Construct file path safely
        let filepath: PathBuf = img_dir.join(sanitize_filename::sanitize(filename));

        // Create file asynchronously
        let mut f = match File::create(&filepath).await {
            Ok(file) => file,
            Err(e) => {
                return HttpResponse::InternalServerError()
                    .content_type("text/plain")
                    .body(format!("Failed to create file: {}", e));
            }
        };

        // Write chunks asynchronously
        while let Some(chunk_result) = field.try_next().await.unwrap_or(None) {
            if let Err(e) = f.write_all(&chunk_result).await {
                return HttpResponse::InternalServerError()
                    .content_type("text/plain")
                    .body(format!("Failed to write to file: {}", e));
            }
        }
    }

    HttpResponse::Ok()
        .content_type("text/plain")
        .body("File has been uploaded")
}
