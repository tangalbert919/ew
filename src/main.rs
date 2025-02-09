mod encryption;
mod router;
mod sql;

use actix_web::{
    App,
    HttpServer,
    get,
    HttpResponse,
    HttpRequest,
    web,
    dev::Service,
    http::header::ContentType
};
use clap::Parser;
use std::fs;

#[get("/index.css")]
async fn css(_req: HttpRequest) -> HttpResponse {
    HttpResponse::Ok()
        .insert_header(ContentType(mime::TEXT_CSS))
        .body(include_file!("webui/dist/index.css"))
}
#[get("/index.js")]
async fn js(_req: HttpRequest) -> HttpResponse {
    HttpResponse::Ok()
        .insert_header(ContentType(mime::APPLICATION_JAVASCRIPT_UTF_8))
        .body(include_file!("webui/dist/index.js"))
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(short, long, default_value_t = 8080, help = "Port to listen on")]
    port: u16,

    #[arg(long, default_value = "./", help = "Path to store database files")]
    path: String,

    #[arg(long, default_value_t = false, help = "Serve gree headers with https. WILL NOT ACCEPT HTTPS REQUESTS")]
    https: bool,

    #[arg(long, default_value = "http://127.0.0.1:51376", help = "Address to NPPS4 server for sif account linking")]
    npps4: String,

    //below options are for the "Help" page

    #[arg(long, default_value = "", help = "Link to patched android global apk for this server.")]
    global_android: String,

    #[arg(long, default_value = "", help = "Link to patched android japan apk for this server.")]
    japan_android: String,

    #[arg(long, default_value = "", help = "Link to patched iOS global apk for this server.")]
    global_ios: String,

    #[arg(long, default_value = "", help = "Link to patched iOS japan apk for this server.")]
    japan_ios: String,

    #[arg(long, default_value = "", help = "Link to asset server.")]
    assets_url: String
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args = get_args();
    let port = args.port;

    let rv = HttpServer::new(|| App::new()
    .wrap_fn(|req, srv| {
        println!("Request: {}", req.path());
        srv.call(req)
    })
    .app_data(web::PayloadConfig::default().limit(1024 * 1024 * 25))
    .service(css)
    .service(js)
    .default_service(web::route().to(router::request))
    ).bind(("0.0.0.0", port))?.run();

    println!("Server started: http://0.0.0.0:{}", port);
    println!("Data path is set to {}", args.path);
    println!("Sif1 transfer requests will attempt to contact NPPS4 at {}", args.npps4);

    if args.https {
        println!("Note: gree is set to https mode. http requests will fail on jp clients.");
    }
    rv.await
}

pub fn get_args() -> Args {
    Args::parse()
}

pub fn get_data_path(file_name: &str) -> String {
    let args = get_args();
    let mut path = args.path;
    while path.ends_with("/") {
        path.pop();
    }
    fs::create_dir_all(&path).unwrap();
    format!("{}/{}", path, file_name)
}

#[macro_export]
macro_rules! include_file {
    ( $s:expr ) => {
        {
            let file = include_flate_codegen::deflate_file!($s);
            let ret = $crate::decode(file);
            std::string::String::from_utf8(ret).unwrap()
        }
    };
}
pub fn decode(bytes: &[u8]) -> Vec<u8> {
    use std::io::{Cursor, Read};

    let mut dec = libflate::deflate::Decoder::new(Cursor::new(bytes));
    let mut ret = Vec::new();
    dec.read_to_end(&mut ret).unwrap();
    ret
}
