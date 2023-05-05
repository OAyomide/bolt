mod utils;

use bolt_http;
use utils::*;

static VERSION: &str = "0.11.11";
static HELP: &str = r#"
Bolt CLI (Build and test APIs)

Usage:
  bolt [OPTIONS]...
  bolt -h | --help
  bolt -v | --version
Options:
  -h --help      Show this screen.
  -v --version   Show version.
  --reset        Reset dist
    "#;

static ADDRESS: &str = "127.0.0.1";

pub fn start(args: Vec<String>, port: u16) {
    let mut args = args;

    args.remove(0);

    let mut is_tauri = false;
    let mut launch = false;
    let mut reset = false;

    match std::env::var_os("BOLT_DEV") {
        Some(_) => {
            reset = true;
        }
        None => {}
    }

    if args.len() > 0 {
        let flag = args[0].as_str();

        match flag {
            "--reset" => reset = true,

            "-h" | "--help" => {
                println!("{}", HELP);
            }

            "-v" | "--version" => {
                println!("bolt {}", VERSION);
            }

            "--tauri" => {
                is_tauri = true;

                launch = true;
            }

            _ => {
                panic!("unknown flag");
            }
        }
    } else {
        launch = true;
    }

    if reset {
        reset_home();
    }

    if launch {
        verify_home();
        verify_state();

        if !is_tauri {
            verify_dist();
        }

        if !is_tauri {
            std::thread::spawn(move || {
                bolt_http::launch_asset_server(port + 1, ADDRESS.to_string());
            });
        }

        std::thread::spawn(move || {
            bolt_ws::launch_server(5555, ADDRESS.to_string());
        });

        bolt_http::launch_server(port, ADDRESS.to_string());
    }
}
