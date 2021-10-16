use std::collections::HashMap;

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
enum GameState {
    Draw,
    XWins,
    OWins,
    InProgress,
}

#[allow(dead_code)]
struct Game {
    board: HashMap<(u8, u8), char>,
    game_state: GameState,
}

#[allow(dead_code)]
impl Game {
    fn new() -> Game {
        Game {
            board: HashMap::new(),
            game_state: GameState::InProgress,
        }
    }

    fn create_board(&mut self, board_size:u8) {
        unimplemented!()
    }

    fn check_game_state(&self) -> GameState {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use std::{convert::TryInto, fs};

    use super::*;

    const TEST_EXAMPLES: &str = "./test_data/gs_examples.json";

    #[test]
    fn test_create_board() {
        for x in 3..6 {
            let mut game = Game::new();
            game.create_board(x);
            let board = &game.board;
            assert_eq!((board.len() as u8), x * x)
        }
    }

    #[test]
    fn test_game_states() {
        let test_file = fs::File::open(TEST_EXAMPLES)
            .expect("Failed to open the file for 'In Progress' testing");
        let test_data: serde_json::Value = serde_json::from_reader(test_file)
            .expect("File for 'In Progress' testing was not a proper JSON");

        let mut game = Game::new();

        for tests in test_data["tests"].as_object().unwrap() {
            let entry = tests.1.as_object().unwrap();
            let board_size: u8 = entry["size"].as_i64().unwrap().try_into().unwrap();
            let board_data = entry["board"].as_array().unwrap();
            let state: GameState = match entry["state"].as_str().unwrap() {
                "in progress" => GameState::InProgress,
                "x wins" => GameState::XWins,
                "o wins" => GameState::OWins,
                "draw" => GameState::Draw,
                _ => panic!("Unidentified game state"),
            };

            let mut board: HashMap<(u8, u8), char> = HashMap::new();
            for (p, v) in board_data.into_iter().enumerate() {
                let i: u8 = p as u8;
                let x: u8 = i % board_size;
                let y: u8 = (i - x) / board_size;
                let c: char = v.as_str().unwrap().chars().next().unwrap();
                board.insert((x, y), c);
            }
            game.board = board;
            assert_eq!(game.check_game_state(), state);
        }
    }
}
