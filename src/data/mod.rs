use std::path::PathBuf;
use std::fs::File;
use std::io::Read;
use actix_web::HttpResponse;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Info{
    pub name: String,
    pub addrip: String,
    pub port: String
}

pub async fn dat(path: PathBuf, types: String) -> actix_web::HttpResponse{
    let mut content = File::open(&path).unwrap();
    // println!("{:?}",&mut content);
    let mut contents = String::new();
    content.read_to_string(&mut contents).unwrap();
    // println!("{:?}",&mut contents);
    HttpResponse::Ok()
        .content_type(format!("{}",types))
        .body(contents)
}

pub async fn get_info() -> Info{
    let mut script = File::open(std::path::PathBuf::from("./config.yaml")).unwrap();
    let mut conten = String::new();
    script.read_to_string(&mut conten).unwrap();
    let info: Info = serde_yaml::from_str(&conten).unwrap();
    info
}