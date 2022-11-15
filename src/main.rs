use actix_web::{web, get, App, HttpServer,HttpRequest, HttpResponse, Error};
// use actix_web_middleware_redirect_scheme::RedirectSchemeBuilder;
use std::path::PathBuf;
use actix_files::NamedFile;
mod data;

// 主页文件
#[get("/")]
async fn index(i: web::Data<data::Info>) -> HttpResponse{ 
    let src = format!("./http/{}",i.html);
    let path: PathBuf = PathBuf::from(src);
    data::dat(path, String::from("text/html")).await
}

// 其他html文件处理
#[get("/{filename}")]
async fn html(req: HttpRequest) -> HttpResponse{ 
    let src = format!("./http/{}",req.match_info().query("filename"));
    // println!("{}",&src);
    let path: PathBuf = PathBuf::from(src);
    data::dat(path, String::from("text/html")).await

}

// css文件处理
#[get("/css/{filename}")]
async fn css(req: HttpRequest, i: web::Data<data::Info>) ->  HttpResponse{
    let src = format!("./http/{}/{}", i.css, req.match_info().query("filename"));
    let path: PathBuf = PathBuf::from(src);
    data::dat(path, String::from("text/css")).await
}

// js文件处理
#[get("/js/{filename}")]
async fn js(req: HttpRequest, i: web::Data<data::Info>) ->  HttpResponse{
    let src = format!("./http/{}/{}", i.js,req.match_info().query("filename"));
    let path: PathBuf = PathBuf::from(src);
    data::dat(path, String::from("text/js")).await
}

// 其他文件处理【视频、图片】普通文件
#[get("/file/{content}/{filename}.{type}")]
async fn files(req: HttpRequest) ->  Result<NamedFile, Error>{
    // println!("{}, {}, {}", req.match_info().query("content"), req.match_info().query("filename"),req.match_info().query("type"));
    let src = format!("./http/file/{}/{}.{}",req.match_info().query("content"), req.match_info().query("filename"),req.match_info().query("type"));
    let path: PathBuf = PathBuf::from(src);
    data::files(path).await
}

// 其他文件处理【视频、图片】vue文件
#[get("/static/{content}/{filename}.{type}")]
async fn files_vue(req: HttpRequest) ->  Result<NamedFile, Error>{
    let src = format!("./http/static/{}/{}.{}",req.match_info().query("content"), req.match_info().query("filename"),req.match_info().query("type"));
    let path: PathBuf = PathBuf::from(src);
    data::files(path).await
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let info = data::get_info().await;
    let i = info.clone();
    println!("【{}】 have running at {}/{}:{}", i.name, i.addrip_local, i.addrip, i.port);
    HttpServer::new(move ||{
        App::new()
            .app_data(web::Data::new(info.clone()))
            .service(index)
            .service(html)
            .service(css)
            .service(js)
            .service(files)
            .service(files_vue)
    })
    .bind(format!("{}:{}", i.addrip_local, i.port))?
    .bind(format!("{}:{}", i.addrip, i.port))?
    .run()
    .await
}
