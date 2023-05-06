mod utils;

use serde::{Deserialize, Serialize};

use std::{
    net::{TcpListener, TcpStream},
    thread::spawn,
};
use tungstenite::Message;
use tungstenite::{
    accept_hdr,
    handshake::server::{Request, Response},
    WebSocket,
};

#[derive(Serialize, Deserialize)]
pub enum MsgType {
    PING,
}

#[derive(Serialize, Deserialize)]
struct ReceivedMessage {
    msg_type: MsgType,
}

#[derive(Serialize, Deserialize)]
struct PingMsg {
    msg_type: MsgType,
    body: String,
}

fn process_message(websocket: &mut WebSocket<TcpStream>, msg: Message) {
    println!("WS SESSION: new message");

    if msg.is_text() {
        let txt = msg.into_text().unwrap();

        let rcv: Result<ReceivedMessage, serde_json::Error> = serde_json::from_str(&txt);

        match rcv {
            Ok(message) => match message.msg_type {
                MsgType::PING => {
                    handle_ping(websocket, txt);
                }
            },

            Err(_err) => {
                handle_invalid(websocket, txt);
            }
        }
    } else {
    }
}

fn handle_ping(websocket: &mut WebSocket<TcpStream>, _txt: String) {
    println!("WS SESSION: received ping");

    let msg = PingMsg {
        msg_type: MsgType::PING,
        body: "pong".to_string(),
    };

    let response = Message::Text(serde_json::to_string(&msg).unwrap());

    websocket.write_message(response).unwrap();
}

fn handle_invalid(websocket: &mut WebSocket<TcpStream>, _txt: String) {
    println!("WS SESSION: received invalid");

    let response = Message::Text("that was invalid".to_string());
    websocket.write_message(response).unwrap();
}

fn process_connection(req: &Request, mut response: Response) -> Response {
    println!("WS: new session with path: {}", req.uri().path());

    // println!("The request's headers are:");
    // for (ref header, _value) in req.headers() {
    // println!("* {}", header);
    // }

    let headers = response.headers_mut();
    headers.append("MyCustomHeader", ":)".parse().unwrap());

    response
}

pub fn launch_server(port: u16, address: String) {
    println!("Starting WS server on {} port {}", address, port);

    let server = TcpListener::bind(address + ":" + &port.to_string()).unwrap();

    for stream in server.incoming() {
        spawn(move || {
            let callback = |req: &Request, response: Response| {
                let response = process_connection(req, response);

                Ok(response)
            };

            let mut websocket = accept_hdr(stream.unwrap(), callback).unwrap();

            loop {
                let msg = websocket.read_message();

                match msg {
                    Ok(msg) => {
                        process_message(&mut websocket, msg);
                    }

                    Err(err) => {
                        println!("WS SESSION: {err}");

                        return;
                    }
                }
            }
        });
    }
}
