use std::path::PathBuf;
use std::fs::File;
use std::io::Read;
use actix_web::HttpResponse;
use serde::{Serialize, Deserialize};
use serde_yaml;
use actix_files::NamedFile;
use actix_web::{ http::header::{ContentDisposition,DispositionType}, Error };

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Info{
    pub name: String,
    pub addrip_local: String,
    pub addrip: String,
    pub port: String,
    pub js: String,
    pub css: String,
    pub html: String
}

pub async fn dat(path: PathBuf, types: String) -> actix_web::HttpResponse{
    let mut content = File::open(&path).unwrap();
    // println!("{:?}",&mut content);
    let mut contents = String::new();
    content.read_to_string(&mut contents).unwrap();
    // println!("{:?}",&mut contents);
    let i = get_info().await;
    HttpResponse::Ok()
        .content_type(format!("{}",types))
        .header("Server",format!("{}",i.name))
        .header("Access-Control-Allow-Origin","*")
        .body(contents)
}

pub async fn get_info() -> Info{
    let mut script = File::open(std::path::PathBuf::from("./config.yaml")).unwrap();
    let mut conten = String::new();
    script.read_to_string(&mut conten).unwrap();
    let info: Info = serde_yaml::from_str(&conten).unwrap();
    info
}

pub async fn files(path: PathBuf) -> Result<NamedFile, Error>{
    let file = NamedFile::open(path)?;
    println!("{:?}", &file);
    Ok(file.use_last_modified(true)
        .set_content_disposition(
            ContentDisposition{
                disposition: DispositionType::Attachment,
                parameters: vec![]
            }
        )
    )
}