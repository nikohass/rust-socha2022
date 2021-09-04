use argparse::{ArgumentParser, Store};
mod xml_client;
use xml_client::XmlClient;
mod xml_node;
//use game_sdk::player::Player;
use player::random::RandomPlayer;

fn main() {
    let mut host = "localhost".to_string();
    let mut port = "13050".to_string();
    let mut reservation = "".to_string();

    {
        let mut parser = ArgumentParser::new();
        parser
            .refer(&mut host)
            .add_option(&["-h", "--host"], Store, "Host");
        parser
            .refer(&mut port)
            .add_option(&["-p", "--port"], Store, "Port");
        parser
            .refer(&mut reservation)
            .add_option(&["-r", "--reservation"], Store, "Reservation");
        parser.parse_args_or_exit();
    }

    let player = Box::new(RandomPlayer::default());

    println!("{}:{} {}", host, port, reservation);
    let mut client = XmlClient::new(host, port, reservation, player);
    client.run();
}
