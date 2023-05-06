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
pub struct PingMsg {
    pub msg_type: MsgType,
    pub body: String,
}

#[derive(Serialize, Deserialize)]
pub struct ReceivedMessage {
    pub msg_type: MsgType,
}

fn process_message(websocket: &mut WebSocket<TcpStream>, session_id: &String, msg: Message) {
    println!("WS {}: new message", session_id);

    if msg.is_text() {
        let txt = msg.into_text().unwrap();

        let rcv: Result<ReceivedMessage, serde_json::Error> = serde_json::from_str(&txt);

        match rcv {
            Ok(message) => match message.msg_type {
                MsgType::PING => {
                    handle_ping(websocket, session_id, txt);
                }
            },

            Err(_err) => {
                handle_invalid(websocket, session_id, txt);
            }
        }
    } else {
    }
}

fn handle_ping(websocket: &mut WebSocket<TcpStream>, session_id: &String, _txt: String) {
    println!("WS {}: received ping", session_id);

    let msg = PingMsg {
        msg_type: MsgType::PING,
        body: "pong".to_string(),
    };

    let response = Message::Text(serde_json::to_string(&msg).unwrap());

    websocket.write_message(response).unwrap();
}

fn handle_invalid(websocket: &mut WebSocket<TcpStream>, session_id: &String, _txt: String) {
    println!("WS {}: received invalid", session_id);

    let response = Message::Text("that was invalid".to_string());
    websocket.write_message(response).unwrap();
}

fn process_connection(req: &Request, mut response: Response, session_id: &String) -> Response {
    println!(
        "WS: new session {} on path: {}",
        session_id,
        req.uri().path()
    );

    // println!("The request's headers are:");
    // for (ref header, _value) in req.headers() {
    // println!("* {}", header);
    // }

    let headers = response.headers_mut();
    headers.append("MyCustomHeader", ":)".parse().unwrap());

    response
}

pub fn launch_ws_server(port: u16, address: String) {
    println!("Starting WS server on {} port {}", address, port);

    let server = TcpListener::bind(address + ":" + &port.to_string()).unwrap();

    for stream in server.incoming() {
        spawn(move || {
            let session_id = uuid::Uuid::new_v4()
                .to_string()
                .splitn(2, '-')
                .next()
                .unwrap()
                .to_string();

            let callback = |req: &Request, response: Response| {
                let response = process_connection(req, response, &session_id);

                Ok(response)
            };

            let mut websocket = accept_hdr(stream.unwrap(), callback).unwrap();

            loop {
                let msg = websocket.read_message();

                match msg {
                    Ok(msg) => {
                        process_message(&mut websocket, &session_id, msg);
                    }

                    Err(err) => {
                        println!("WS {}: {}", &session_id, err);

                        return;
                    }
                }
            }
        });
    }
}
