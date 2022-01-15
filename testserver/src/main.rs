use game_sdk::action::Action;
use game_sdk::gamerules;
use game_sdk::gamestate::{GameState, BLUE, RED};
use game_sdk::player::Player;
use std::fmt::{Display, Formatter, Result};
use std::io::{BufRead, BufReader, Write};
use std::process::{ChildStdin, ChildStdout, Command, Stdio};
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};

pub struct TestResult {
    games_played: usize,
    one: usize,
    draw: usize,
    two: usize,
}

impl Default for TestResult {
    fn default() -> Self {
        Self {
            games_played: 0,
            one: 0,
            draw: 0,
            two: 0,
        }
    }
}

impl Display for TestResult {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "Games: {:6} One: {:6} Draw: {:6} Two: {:6}",
            self.games_played, self.one, self.draw, self.two
        )
    }
}

impl TestResult {
    pub fn add_game_result(&mut self, result: i16) {
        self.games_played += 1;
        match result {
            0 => self.draw += 1,
            r if r > 0 => self.one += 1,
            _ => self.two += 1,
        };
    }
}

pub struct ClientInstance {
    stdin: ChildStdin,
    stdout: ChildStdout,
}

impl ClientInstance {
    pub fn new(path: String) -> Self {
        let mut process = Command::new(path.clone())
            .args(&["--test", "true"])
            //.args(&["--time", "200"])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .unwrap_or_else(|_| panic!("Can't start the client: {}", path));
        Self {
            stdin: process.stdin.take().unwrap(),
            stdout: process.stdout.take().unwrap(),
        }
    }
}

impl Player for ClientInstance {
    fn on_move_request(&mut self, state: &GameState) -> Action {
        let mut fen = state.to_fen();
        fen.push('\n');
        self.stdin.write_all(fen.as_bytes()).unwrap();
        let mut read = BufReader::new(&mut self.stdout);
        let mut line = String::new();
        loop {
            read.read_line(&mut line).unwrap();
            if !line.is_empty() && line.contains("action: ") {
                line = (&line[8..]).to_string();
                break;
            }
            if !line.is_empty() {
                line.pop();
                //println!("info: {}", line);
            }
            line.truncate(0);
        }
        line.pop();
        Action::deserialize(line)
    }

    fn reset(&mut self) {
        self.stdin
            .write_all(String::from("reset\n").as_bytes())
            .unwrap();
        let mut read = BufReader::new(&mut self.stdout);
        let mut line = String::new();
        loop {
            read.read_line(&mut line).unwrap();
            if !line.is_empty() && line.contains("reset") {
                return;
            }
            line.truncate(0);
        }
    }
}

pub fn run_test(
    client_one: String,
    client_two: String,
    test_result: Arc<Mutex<TestResult>>,
) -> JoinHandle<()> {
    let mut instance_one = ClientInstance::new(client_one);
    let mut instance_two = ClientInstance::new(client_two);
    let mut first_player = RED;
    thread::spawn(move || loop {
        let mut state = GameState::random();
        while !gamerules::is_game_over(&state) {
            let action = if state.ply as usize % 2 == first_player {
                instance_one.on_move_request(&state)
            } else {
                instance_two.on_move_request(&state)
            };
            gamerules::do_action(&mut state, action);
        }
        let mut game_result = gamerules::game_result(&state);
        if first_player == BLUE {
            game_result *= -1;
        }
        let mut r = test_result.lock().unwrap();
        r.add_game_result(game_result);
        println!("{}", r);
        first_player = match first_player {
            RED => BLUE,
            _ => RED,
        };
        instance_one.reset();
        instance_two.reset();
    })
}

fn main() {
    let path_one = String::from("target/release/client.exe");
    let path_two = String::from("clients/cb.exe");
    let threads: usize = 6;

    let test_result = Arc::new(Mutex::new(TestResult::default()));
    let mut handles: Vec<JoinHandle<()>> = Vec::with_capacity(8);
    for _ in 0..threads {
        handles.push(run_test(
            path_one.clone(),
            path_two.clone(),
            Arc::clone(&test_result),
        ));
    }
    for handle in handles {
        handle.join().unwrap();
    }
    println!("Result: {}", *test_result.lock().unwrap());
}
