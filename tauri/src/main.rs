// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod utils;

use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use tauri::Window;
use crate::utils::*;

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
pub enum Method {
    GET,
    POST,
    PUT,
    DELETE,
    HEAD,
    PATCH,
    OPTIONS,
    CONNECT,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
pub enum ResponseType {
    TEXT,
    JSON,
}

#[derive(Clone, Serialize)]
struct HttpResponse {
    status: u16,
    body: String,
    headers: Vec<Vec<String>>,
    time: u32,
    size: u64,
    response_type: ResponseType,
    request_index: usize,
    failed: bool,
}

impl HttpResponse {
    fn new() -> Self {
        HttpResponse {
            status: 0,
            body: String::new(),
            headers: Vec::new(),
            time: 0,
            size: 0,
            response_type: ResponseType::TEXT,
            request_index: 0,
            failed: false,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct HttpRequest {
    url: String,
    method: Method,
    body: String,
    headers: Vec<Vec<String>>,
    request_index: usize,
}

#[derive(Serialize)]
struct AppState {
    response: HttpResponse,
}

impl AppState {
    fn new() -> Self {
        Self {
            response: HttpResponse::new(),
        }
    }
}

// Create a shared global state variable
lazy_static::lazy_static! {
    static ref GLOBAL_STATE: Arc<Mutex<AppState>> = Arc::new(Mutex::new(AppState::new()));
}

#[tauri::command]
fn bolt_log(log: &str) -> String {
    println!("{}", log);

    return "done".to_string();
}

#[tauri::command]
fn bolt_panic(log: &str) {
    panic!("{}", log);
}

#[tauri::command]
fn open_link(link: String) {
    webbrowser::open(&link).unwrap();
}

#[tauri::command]
fn send_request(
    window: Window,
    url: String,
    method: Method,
    body: String,
    headers: Vec<Vec<String>>,
    index: usize,
) -> String {
    // bolt_log("Sending request");

    let req = HttpRequest {
        url,
        method,
        body,
        headers,
        request_index: index,
    };

    // bolt_log(&format!("{:?}", req));

    std::thread::spawn(move || {
        let resp: HttpResponse = http_send(req);

        let resp = serde_json::to_string(&resp).unwrap();

        window.emit("receive_response", resp).unwrap();
    });

    return "done".to_string();
}

fn http_send(mut req: HttpRequest) -> HttpResponse {
    if !req.url.contains("http") {
        let new_url = "http://".to_string() + &req.url;

        req.url = new_url;
    }

    // bolt_log(&req.url);

    let mut request = prepare_request(req.clone());

    for h in req.headers {
        if h[0] != "" && h[1] != "" {
            println!("{} : {}", h[0], h[1]);
            request = request.header(h[0].clone(), h[1].clone());
        }
    }

    let start = get_timestamp();
    let response = request.send();
    let end = get_timestamp();

    let mut http_response = match response {
        Ok(resp) => {
            let mut new_response = HttpResponse::new();

            new_response.headers = extract_headers(resp.headers());
            new_response.status = resp.status().as_u16();
            new_response.time = (end - start) as u32;
            new_response.body = resp.text().unwrap();
            new_response.size = new_response.body.len() as u64;

            if new_response.headers.contains(&vec![
                "content-type".to_string(),
                "application/json".to_string(),
            ]) {
                new_response.response_type = ResponseType::JSON;
            }

            new_response
        }

        Err(err) => {
            let mut err_resp = HttpResponse::new();

            err_resp.failed = true;

            err_resp.body = err.to_string();

            err_resp
        }
    };

    http_response.request_index = req.request_index;

    let mut state = GLOBAL_STATE.lock().unwrap();
    state.response = http_response.clone();

    return http_response;
}

fn prepare_request(req: HttpRequest) -> reqwest::blocking::RequestBuilder {
    let client = reqwest::blocking::Client::new();

    let builder = match req.method {
        Method::GET => client.get(req.url).body(req.body),
        Method::POST => client.post(req.url).body(req.body),
        Method::PUT => client.put(req.url).body(req.body),
        Method::DELETE => client.delete(req.url).body(req.body),
        Method::HEAD => client.head(req.url).body(req.body),
        Method::PATCH => client.patch(req.url).body(req.body),
        Method::OPTIONS => client
            .request(reqwest::Method::OPTIONS, req.url)
            .body(req.body),
        Method::CONNECT => client
            .request(reqwest::Method::CONNECT, req.url)
            .body(req.body),
    };

    return builder;
}


#[tauri::command]
fn save_state(save: &str) -> String {
    // println!("{}", save);
    std::fs::write(get_home() + "state.json", save).unwrap();

    return "done".to_string();
}

#[tauri::command]
fn restore_state() -> String {
    // println!("{}", save);
    let state = std::fs::read_to_string(get_home() + "state.json").unwrap();

    return state;
}

fn main() {
    verify_home();
    verify_state();

    let app = tauri::Builder::default().invoke_handler(tauri::generate_handler![
        send_request,
        open_link,
        bolt_log,
        bolt_panic,
        save_state,
        restore_state
    ]);

    app.run(tauri::generate_context!()).unwrap();
}

