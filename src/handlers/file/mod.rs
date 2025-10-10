use std::{io::Write, path::PathBuf};
pub mod file_async;

use actix_multipart::{form::{json::Json as MPJson, tempfile::TempFile, MultipartForm}, Multipart};
use actix_web::{get, http::header::ContentDisposition, post, web::{self, Payload}, Error, HttpRequest, HttpResponse, Responder};
use actix_files as fs;
use futures_util::{StreamExt, TryStreamExt};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Metadata {
  name: String
}
#[derive(Debug, MultipartForm)]
pub struct UploadForm{
  #[multipart(limit = "10MB")]
  file: TempFile,
  json: MPJson<Metadata>
}

#[post("/upload")]
pub async fn upload_video(MultipartForm(form): MultipartForm<UploadForm>) -> impl Responder {
  format!("You have uploaded file with name {} and size {}", form.json.name, form.file.size)
}

//uploading a file from terminal
// mk@Root:~/Documents/actix-web/actix-mod$ curl -v --request POST \
// > --url http://localhost:8080/upload \
// > -F 'json={"name": "Cargo.lock"}; type=application/json' \
// > -F file=@./Cargo.lock

#[get("/download/{filename:.*}")]
pub async fn download_file(req:HttpRequest) -> Result<fs::NamedFile, Error> {
  let current_dir = std::env::current_dir().expect("failed to read current directory");
  println!("current directory: {}", current_dir.display());

  let path: std::path::PathBuf = req.match_info()
  .query("filename")
  .parse()
  .unwrap();

  let file = fs::NamedFile::open(path)?;

  Ok(file
    .use_last_modified(true)
    .set_content_disposition(ContentDisposition{
      disposition: actix_web::http::header::DispositionType::Attachment,
      parameters: vec![],
    })
  )
}


#[post("/v1/upload")]
pub async fn uploadv1(mut payload: Multipart) -> actix_web::Result<HttpResponse> {
  let current_dir = std::env::current_dir().expect("failed to read current directory");
  let img_dir = current_dir.join("images");

  match std::fs::create_dir_all(&img_dir) {
    Ok(_) => println!("image dir was created successfully"),
    Err(err) => {
      if err.kind() == std::io::ErrorKind::AlreadyExists {
        println!("image dir already exists: {:?}", img_dir)
      }else {
        println!("Error creating image dir. error: {}", err)
      }
    }
  }

  let img_path = img_dir.join("file.png");
  let mut file = std::fs::File::create(img_path)?;

  while let Ok(Some(mut field)) = payload.try_next().await {
    let content_disposition = field.content_disposition().unwrap();
    let filename = content_disposition.get_filename().unwrap_or_else(|| "sample.png").to_string();

    if !filename.is_empty() {
      while let Some(chunk) = field.next().await {
        let c = chunk?.to_vec();
        let mut data: &[u8] = c.as_slice();

        std::io::copy(&mut data, &mut file).unwrap();
      }
    }
  }

  Ok(HttpResponse::Ok().body("file has been successfully uploaded"))
}


#[post("/v2/upload")]
pub async fn uploadv2(mut payload: Multipart) -> impl Responder {
  let current_dir = std::env::current_dir().expect("failed to read current directory");
  let img_dir = current_dir.join("images");

  while let Ok(Some(mut field)) = payload.try_next().await {
    let content_disposition = field.content_disposition().unwrap();
    let filename = content_disposition.get_filename().unwrap_or_else(|| "sample.png").to_string();
    let filepath: PathBuf = img_dir.join(sanitize_filename::sanitize(filename));

    let f = web::block(|| std::fs::File::create(filepath))
    .await
    .unwrap();

    match f {
      Ok(mut f) => {
        while let Some(chunk) = field.next().await {
          let data = chunk.unwrap();
          let _ = f.write_all(&data);
        }
      },
      Err(e) => return HttpResponse::BadRequest().content_type("text/plain").body(e.to_string())
    }
  }

  HttpResponse::Ok().content_type("text/plain").body("File has been uploaded")
}