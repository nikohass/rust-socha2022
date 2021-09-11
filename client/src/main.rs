use argparse::{ArgumentParser, Store};
mod xml_client;
use xml_client::XmlClient;
mod xml_node;
use game_sdk::gamestate::GameState;
use game_sdk::player::Player;
use player::random::RandomPlayer;

fn run_test(mut player: Box<dyn Player>) {
    loop {
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Can't read line");
        input.pop();
        if (input[..5]).to_string() == "reset" {
            player.reset();
            println!("reset");
            continue;
        }
        let state = GameState::from_fen(&input.clone());
        let action = player.on_move_request(&state);
        println!("action: {}", action.serialize());
    }
}

fn main() {
    let mut host = "localhost".to_string();
    let mut port = "13050".to_string();
    let mut reservation = "".to_string();
    let mut test = false;

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
        parser
            .refer(&mut test)
            .add_option(&["-t", "--test"], Store, "Test");
        parser.parse_args_or_exit();
    }

    let player = Box::new(RandomPlayer::default());
    if test {
        run_test(player);
    } else {
        println!("{}:{} {}", host, port, reservation);
        let mut client = XmlClient::new(host, port, reservation, player);
        client.run();
    }
}
