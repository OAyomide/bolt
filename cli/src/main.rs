mod utils;

use actix_web::{body, http, web, App, HttpRequest, HttpResponse, HttpServer};
use serde::{Deserialize, Serialize};
use utils::*;

#[derive(Serialize, Deserialize)]
struct Ping {
    body: String,
}

static VERSION: &str = "0.11.1";
static HELP: &str = r#"
Bolt CLI (Build and test APIs)
Usage:
  bolt-cli [OPTIONS]...
  bolt-cli -h | --help
  bolt-cli -v | --version
Options:
  -h --help      Show this screen.
  -v --version   Show version.
  --reset        Download latest dist
    "#;

#[actix_web::get("/ping")]
async fn ping(req: HttpRequest, body: String) -> HttpResponse {
    println!("GET {}", req.uri());

    println!("Body: \n{body}");

    println!("Headers: ");

    for (key, value) in req.headers().iter() {
        println!("{} => {}", key.as_str(), value.to_str().unwrap());
    }

    println!("------------------------------------------------------");

    let body = Ping {
        body: "pong".to_string(),
    };

    let response = HttpResponse::Ok().json(body);

    return response;
}

async fn e404(_req: HttpRequest) -> HttpResponse {
    println!("404 was called");

    let body = body::BoxBody::new("Not Found");
    let response: HttpResponse = HttpResponse::new(http::StatusCode::NOT_FOUND).set_body(body);

    return response;
}

fn main() {
    let mut args: Vec<String> = std::env::args().collect();
    args.remove(0);

    #[cfg(debug_assertions)]
    reset_home();

    verify_home();
    verify_dist();

    if args.len() == 0 {
        launch_server().unwrap();
    } else {
        let flag = args[0].as_str();

        match flag {
            "--reset" => reset_home(), // delete the book and download a fresh copy

            "-h" | "--help" => {
                println!("{}", HELP);
            }

            "-v" | "--version" => {
                println!("thebook {}", VERSION);
            }

            _ => {
                panic!("unknown flag");
            }
        }
    }
}

#[actix_web::main]
async fn launch_server() -> std::io::Result<()> {
    std::thread::spawn(|| {
        open_browser("http://localhost:4455".to_string());
    });

    let app = HttpServer::new(|| {
        let dist_path = get_dist();

        App::new()
            .service(ping)
            .service(actix_files::Files::new("/", dist_path).index_file("index.html"))
            .default_service(web::post().to(e404))
    });

    let port: u16 = 4455;
    let address = "0.0.0.0";

    println!("Starting server on {} port {}", address, port);
    app.bind((address, port)).unwrap().run().await
}

// downloads the dist's source files from the official github repository and builds it
pub fn build_dist() {
    println!("Downloading static files");

    #[cfg(debug_assertions)]
    _clone_repo_debug();

    #[cfg(not(debug_assertions))]
    _clone_repo_release();

    let shell_command = format!("cp -r ./dist/ ../../dist");
    run_command(shell_command, get_home() + "bolt/cli/");

    let shell_command = format!("cp ./bolt/icon/* ./dist/");
    run_command(shell_command, get_home());

    println!("Download complete");
}

fn _clone_repo_debug() {
    let shell_command = format!("rsync -a --exclude-from=.gitignore --exclude='.git' ./ {}", get_home() + "bolt/");
    run_command(shell_command, "../".to_string());
}

fn _clone_repo_release() {
    let url = "https://github.com/hiro-codes/bolt";

    let shell_command = format!("git clone {url} --depth 1");

    run_command(shell_command, get_home());
}
