use std::path::Path;
use std::time::SystemTime;

pub fn extract_headers(map: &reqwest::header::HeaderMap) -> Vec<Vec<String>> {
    let mut headers: Vec<Vec<String>> = Vec::new();

    for (key, value) in map.iter() {
        let mut header: Vec<String> = Vec::new();

        header.push(key.to_string());
        header.push(value.to_str().unwrap().to_string());

        headers.push(header);
    }

    return headers;
}

pub fn verify_home() {
    let path = get_home();
    if !dir_exists(&path) {
        println!("Creating directory {}", path);
        create_home(&path);
    }
}

pub fn verify_state() {
    let path = get_home() + "state.json";
    if !file_exists(&path) {
        println!("Creating state file");

        create_state(&path);
    }
}

pub fn create_home(path: &String) {
    std::fs::create_dir(path).unwrap();
}

pub fn create_state(path: &String) {
    let new_state = r#"
{"page":"Collections","main_current":0,"col_current":[0,0],"main_col":{"name":"New Collection ","requests":[{"url":"","body":"","headers":[["",""]],"params":[["",""]],"method":"GET","response":{"status":0,"body":"","headers":[],"time":0,"size":0,"response_type":"TEXT","request_index":0,"failed":false},"name":"New Request ","req_tab":1,"resp_tab":1}],"collapsed":false},"collections":[]}
"#;

    std::fs::write(path, new_state).unwrap();
}

pub fn get_timestamp() -> u128 {
    return SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_millis();
}

pub fn dir_exists(path: &String) -> bool {
    if Path::new(&path).exists() {
        true
    } else {
        println!("Directory {} does not exist", path);
        false
    }
}

pub fn file_exists(path: &String) -> bool {
    if Path::new(&path).exists() {
        true
    } else {
        println!("File {} does not exist", path);
        false
    }
}

pub fn get_home() -> String {
    let path = dirs::home_dir().unwrap().to_str().unwrap().to_string() + "/bolt/";
    path
}
