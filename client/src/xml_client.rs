use super::xml_node::XmlNode;
//use game_sdk::action::ActionList;
//use game_sdk::gamerules;
use game_sdk::gamestate::GameState;
use game_sdk::player::Player;
use std::io::{prelude::Write, BufReader, BufWriter};
use std::net::TcpStream;
use xml::reader::*;

pub struct XmlClient {
    host: String,
    port: String,
    reservation: String,
    state: GameState,
    player: Box<dyn Player>,
    room_id: Option<String>,
}

impl XmlClient {
    pub fn new(host: String, port: String, reservation: String, player: Box<dyn Player>) -> Self {
        Self {
            host,
            port,
            reservation,
            state: GameState::empty(),
            player,
            room_id: None,
        }
    }

    pub fn run(&mut self) {
        print!("Connecting to {}:{}... ", self.host, self.port);
        let stream = TcpStream::connect(&format!("{}:{}", self.host, self.port))
            .expect("Could not connect to server");
        println!("Connected");
        Self::write_to(&stream, "<protocol>");
        let join_xml = match self.reservation.as_str() {
            "" => "<join gameType=\"swc_2022\"/>".to_string(),
            _ => format!("<joinPrepared reservationCode=\"{}\" />", self.reservation),
        };
        print!("Sending join message ");
        Self::write_to(&stream, join_xml.as_str());
        self.handle_stream(&stream);
    }

    fn handle_stream(&mut self, stream: &TcpStream) {
        let mut parser = EventReader::new(BufReader::new(stream));
        loop {
            let node = XmlNode::read_from(&mut parser);
            match node.name.as_str() {
                "data" => {
                    let invalid = &"".to_string();
                    let data_class = node.get_attribute("class").unwrap_or(invalid).to_string();
                    match data_class.as_str() {
                        "memento" => {
                            println!("Received memento.");
                            node.as_memento(&mut self.state);
                            println!("{}", self.state.to_fen());
                            println!("{}", self.state);
                        }
                        "welcomeMessage" => {
                            println!("Received welcome message.");
                        }
                        "moveRequest" => {
                            println!("Received move request.");
                            let action = self.player.on_move_request(&self.state);
                            println!("Sending move: {}", action);
                            let xml_move = action.to_xml();
                            Self::write_to(
                                stream,
                                &format!(
                                    "<room roomId=\"{}\">\n{}\n</room>",
                                    self.room_id.as_ref().expect("Error while reading room id"),
                                    xml_move
                                ),
                            );
                        }
                        "result" => {
                            println!("Received result.");
                            return;
                        }
                        s => {
                            println!("{} {}", s, node.data);
                        }
                    }
                }
                "joined" => {
                    self.room_id = Some(node.as_room());
                    println!("Joined {}", node.as_room());
                }
                "sc.protocol.responses.CloseConnection" => {
                    println!("Connection closed");
                    break;
                }
                _ => {}
            }
        }
    }

    fn write_to(stream: &TcpStream, data: &str) {
        let _ = BufWriter::new(stream).write(data.as_bytes());
    }
}
