use game_sdk::gamestate::GameState;
use game_sdk::piece::Piece;
use std::collections::{HashMap, VecDeque};
use std::io::BufReader;
use std::net::TcpStream;
use xml::reader::{EventReader, XmlEvent};

#[derive(Debug)]
pub struct XmlNode {
    pub name: String,
    pub data: String,
    attribs: HashMap<String, Vec<String>>,
    childs: Vec<XmlNode>,
}

impl XmlNode {
    pub fn new() -> Self {
        Self {
            name: String::new(),
            data: String::new(),
            attribs: HashMap::new(),
            childs: Vec::new(),
        }
    }

    pub fn read_from(xml_parser: &mut EventReader<BufReader<&TcpStream>>) -> Self {
        let mut node_stack: VecDeque<XmlNode> = VecDeque::new();
        let mut has_received_first = false;
        let mut final_node: Option<XmlNode> = None;

        loop {
            match xml_parser.next() {
                Ok(XmlEvent::StartElement {
                    name, attributes, ..
                }) => {
                    let mut node = XmlNode::new();
                    node.name = name.local_name;
                    for attribute in attributes {
                        let attrib_name = attribute.name.local_name;
                        if !node.attribs.contains_key(&attrib_name) {
                            node.attribs.insert(attrib_name.to_string(), Vec::new());
                        }
                        node.attribs
                            .get_mut(&attrib_name)
                            .unwrap()
                            .push(attribute.value.to_string());
                    }
                    //println!("{}", node.data);
                    node_stack.push_back(node);
                    has_received_first = true;
                }
                Ok(XmlEvent::EndElement { .. }) => {
                    if node_stack.len() > 2 {
                        let child = node_stack.pop_back().expect("Unexpectedly found empty XML node stack while trying to pop off new child element");
                        let mut node = node_stack.pop_back().expect("Unexpectedly found empty XML node stack while trying to hook up new child element");
                        node.childs.push(child);
                        node_stack.push_back(node);
                    } else if has_received_first {
                        final_node = Some(node_stack.pop_back().expect(
                            "Unexpectedly found empty XML node stack while trying to return node",
                        ));
                    }
                }
                Ok(XmlEvent::Characters(content)) => {
                    node_stack.back_mut().expect("Unexpectedly found empty XML node stack while trying to add characters").data += content.as_str();
                }
                Err(err) => {
                    println!("{:?}", err);
                    break;
                }
                _ => {}
            }
            if final_node.is_some() {
                break;
            }
        }
        final_node.unwrap()
    }

    pub fn as_room(&self) -> String {
        let err = "Error while parsing XML node to Room";
        self.get_attribute("roomId").expect(err).to_string()
    }

    pub fn as_memento(&self, state: &mut GameState) {
        let err = "Error while parsing XML node to Memento";
        self.get_child("state").expect(err).update_state(state);
    }

    pub fn update_state(&self, new_state: &mut GameState) {
        //let mut new_state = GameState::empty();
        new_state.board = [[0u64; 4]; 2];
        new_state.occupied = [0u64; 2];
        new_state.ply = self
            .get_attribute("turn")
            .expect("Error while reading turn")
            .parse::<u8>()
            .expect("Error while parsing turn");

        for entry in (self
            .get_child("board")
            .expect("Error while reading board")
            .get_child("pieces")
            .expect("Error while reading pieces")
            .get_children())
        .iter()
        {
            let coords = entry
                .get_child("coordinates")
                .expect("Error while reading coordinates");
            let x = coords
                .get_attribute("x")
                .expect("Error while reading x coordinate")
                .parse::<usize>()
                .expect("Error while parsing X coordinate");
            let y = coords
                .get_attribute("y")
                .expect("Error while reading y coordinate")
                .parse::<usize>()
                .expect("Error while parsing Y coordinate");
            let p = entry.get_child("piece").expect("Error while reading piece");
            let piece = match p
                .get_attribute("type")
                .expect("Error while reading piece type")
                .as_str()
            {
                "Herzmuschel" => Piece::Cockle,
                "Moewe" => Piece::Gull,
                "Seestern" => Piece::Starfish,
                _ => Piece::Seal,
            };
            let color = match p
                .get_attribute("team")
                .expect("Error while reading team")
                .as_str()
            {
                "ONE" => 0,
                _ => 1,
            };
            let bit = 1 << (x + y * 8);
            new_state.board[color][piece as usize] |= bit;
            new_state.occupied[color] |= bit;
            //println!("{} {} {} {}", x, y, pt, color);
        }
        println!("{}", new_state);
    }

    pub fn get_children(&self) -> &Vec<XmlNode> {
        &self.childs
    }

    pub fn get_child(&self, name: &str) -> Option<&XmlNode> {
        for child in &self.childs {
            if child.name.as_str() == name {
                return Some(child);
            }
        }
        None
    }

    pub fn get_attribute(&self, name: &str) -> Option<&String> {
        self.attribs.get(name).map(|a| &a[0])
    }
}
