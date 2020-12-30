use std::net::TcpListener;
use tungstenite::server::accept;

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub struct ServerState {
    ip_v4: &'static str,
}
/// Create an isntation of the server that will communicate with the client
/// 
impl ServerState {
    pub fn new(ip_v4: &'static str) -> Self {
        Self { ip_v4 }
    }
/// The function that constitutes the core of the library, sends the code to the client with the given "name" and returns the result.    
/// The first argument is the code you want to send, the second is the identifier ("name") and the language in which the code was written.
/// 
/// 
    pub fn run_server(
        self,
        message: &'static str,
        name_device: &'static str,
        leanguage: &'static str,
    ) -> Result<String, tungstenite::Error> {
        let mut get_message = String::from(" ");
        'socket_loop: loop {
            let server = TcpListener::bind(self.ip_v4).expect("don't have permission to this IP or Port");

            'main_loop: for stream in server.incoming() {
                let mut websocket = match accept(stream?) {
                    Ok(s) => s,
                    Err(error) => {
                        println!("Error: {:?} ", error);
                        continue 'socket_loop;
                    }
                };

                loop {
                    let msg = websocket.read_message()?;

                    #[cfg(feature = "socket_debug")]
                    println!("{:?}", msg);

                    get_message = msg.to_text().unwrap_or("error cast message").to_string();
                    if msg.is_binary() || msg.is_text() {
                        if msg.to_text()? == name_device.to_string() {
                            websocket
                                .write_message(tungstenite::Message::Text(leanguage.to_string()))?;
                            websocket
                                .write_message(tungstenite::Message::Text(message.to_string()))?;
                        } else {
                            break 'main_loop;
                        }
                    }
                }
            }

            if get_message != name_device.to_string() {
                if get_message.find(name_device) == Some(0) {
                    let (_, now) =  get_message.split_at(name_device.len());
                    get_message = now.to_string();
                    
                    break 'socket_loop;
                }
            }
        }
        Ok(get_message)
    }

    pub fn run_server_wasm(
        self,
        name_device: &'static str,
    ) -> Result<String, tungstenite::Error> {
        let mut get_message = String::from(" ");
        'socket_loop: loop {
            let server = TcpListener::bind(self.ip_v4).expect("don't have permission to this IP or Port");

            'main_loop: for stream in server.incoming() {
                let mut websocket = match accept(stream?) {
                    Ok(s) => s,
                    Err(error) => {
                        println!("Error: {:?} ", error);
                        continue 'socket_loop;
                    }
                };

                loop {
                    let msg = websocket.read_message()?;

                    #[cfg(feature = "socket_debug")]
                    println!("{:?}", msg);

                    get_message = msg.to_text().unwrap_or("error cast message").to_string();
                    if msg.is_binary() || msg.is_text() {
                        if msg.to_text()? == name_device.to_string() {
                            websocket
                                .write_message(tungstenite::Message::Text("wasm".to_string()))?;
                            websocket
                                .write_message(tungstenite::Message::Text("run_wasm".to_string()))?;
                        } else {
                            break 'main_loop;
                        }
                    }
                }
            }

            if get_message != name_device.to_string() {
                if get_message.find(name_device) == Some(0) {
                    let (_, now) =  get_message.split_at(name_device.len());
                    get_message = now.to_string();
                    
                    break 'socket_loop;
                }
            }
        }
        Ok(get_message)
    }
}
