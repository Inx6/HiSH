// use actix_files::NamedFile;
use actix_web::{get, App, HttpServer,HttpRequest, HttpResponse, http::header::{ContentDisposition,DispositionType}};
// use actix_web_middleware_redirect_scheme::RedirectSchemeBuilder;
use std::path::PathBuf;
mod data;

#[get("/")]
async fn index() -> HttpResponse{ 
    let src = format!("./http/index.html");
    let path: PathBuf = PathBuf::from(src);
    data::dat(path, String::from("text/html")).await
}

#[get("/{filename}")]
async fn html(req: HttpRequest) ->  HttpResponse{ 
    // println!("{:?}",&req.match_info().query("filename"));

    let src = format!("./http/{}",req.match_info().query("filename"));
    let path: PathBuf = PathBuf::from(src);
    data::dat(path, String::from("text/html")).await

    // let file = NamedFile::open(path)?;
    // Ok(file
    //     .use_last_modified(true)
    //     .set_content_disposition(
    //         ContentDisposition{
    //             disposition: DispositionType::Attachment,
    //             parameters: vec![]
    //         }
    //     )
    // )
}

#[get("/css/{filename}")]
async fn css(req: HttpRequest) ->  HttpResponse{
    let src = format!("./http/static/css/{}",req.match_info().query("filename"));
    let path: PathBuf = PathBuf::from(src);

    data::dat(path, String::from("text/css")).await
}

#[get("/js/{filename}")]
async fn js(req: HttpRequest) ->  HttpResponse{
    let src = format!("./http/static/js/{}",req.match_info().query("filename"));
    let path: PathBuf = PathBuf::from(src);

    data::dat(path, String::from("text/js")).await
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let info = data::get_info().await;
    println!("The server【{}】 have running at {}:{}", info.name, info.addrip, info.port);
    HttpServer::new(||{
        App::new()
            .service(index)
            .service(html)
            .service(css)
            .service(js)
    })
    .bind(format!("{}:{}", info.addrip, info.port))?
    .run()
    .await
}
