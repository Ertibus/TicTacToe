use std::collections::HashMap;

#[derive(Debug, PartialEq)]
enum Player {
    X,
    O,
}

#[derive(Debug, PartialEq)]
pub enum GameState {
    Draw,
    Win,
    InProgress,
}

const VICTORY_CONDITION: u8 = 2;

pub struct Game {
    board: HashMap<(u8, u8), char>,
    board_size: u8,
    player: Player,
}

impl Game {
    pub fn new() -> Self {
        Game {
            board: HashMap::new(),
            board_size: 0,
            player: Player::X,
        }
    }

    pub fn get_next_player_symbol(&self) -> char {
        match self.player {
            Player::X => 'X',
            Player::O => 'O',
        }
    }

    pub fn get_prev_player_symbol(&self) -> char {
        match self.player {
            Player::X => 'O',
            Player::O => 'X',
        }
    }

    pub fn create_board(&mut self, board_size:u8) {
        self.board.clear();
        self.board_size = board_size;
        for x in 0..board_size {
            for y in 0..board_size {
                self.board.insert((x, y), ' ');
            }
        }
    }

    pub fn player_input(&mut self, x:u8, y:u8) -> GameState{
        let c: &char;
        if self.player == Player::X {
            c = &'X';
            self.player = Player::O;
        } else {
            c = &'O';
            self.player = Player::X;
        }

        if let Some(b) = self.board.get_mut(&(x, y)) {
            *b = *c;
        }

        self.check_game_state()
    }

    pub fn check_game_state(&self) -> GameState {
        let mut draw: bool = true;

        for x in 0..self.board_size {
            for y in 0..self.board_size {
                if self.board.get(&(x, y)).unwrap() == &' ' {
                    draw = false;
                    break
                }
            }
            if !draw { break; }
        }
        if draw { return GameState::Draw }

        for x in 0..(self.board_size - VICTORY_CONDITION) {
            for y in 0..self.board_size {
                if self.board.get(&(x, y)).unwrap() != &' '
                    && self.board.get(&(x, y)).unwrap() == self.board.get(&(x + 1, y)).unwrap()
                    && self.board.get(&(x + 1, y)).unwrap() == self.board.get(&(x + 2, y)).unwrap() {
                    return GameState::Win;
                }
            }
        }
        for x in 0..self.board_size {
            for y in 0..(self.board_size - VICTORY_CONDITION) {
                if self.board.get(&(x, y)).unwrap() != &' '
                    && self.board.get(&(x, y)).unwrap() == self.board.get(&(x, y + 1)).unwrap()
                    && self.board.get(&(x, y + 1)).unwrap() == self.board.get(&(x, y + 2)).unwrap() {
                    return GameState::Win;
                }
            }
        }

        for x in 0..(self.board_size - VICTORY_CONDITION) {
            for y in 0..(self.board_size - VICTORY_CONDITION) {
                if self.board.get(&(x, y)).unwrap() != &' '
                    && self.board.get(&(x, y)).unwrap() == self.board.get(&(x + 1, y + 1)).unwrap()
                    && self.board.get(&(x + 1, y + 1)).unwrap() == self.board.get(&(x + 2, y + 2)).unwrap() {
                    return GameState::Win;
                }
            }
        }

        for x in 0..(self.board_size - VICTORY_CONDITION) {
            for y in 2..self.board_size {
                if self.board.get(&(x, y)).unwrap() != &' '
                    && self.board.get(&(x, y)).unwrap() == self.board.get(&(x + 1, y - 1)).unwrap()
                    && self.board.get(&(x + 1, y - 1)).unwrap() == self.board.get(&(x + 2, y - 2)).unwrap() {
                    return GameState::Win;
                }
            }
        }
        GameState::InProgress
    }
}

impl Default for Game {
    fn default() -> Self {
        Self::new()
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
    fn test_player_input() {
        let mut game = Game::new();
        game.create_board(3);
        assert_eq!(game.player_input(0, 0), GameState::InProgress);
        assert_eq!(game.player_input(0, 1), GameState::InProgress);
        assert_eq!(game.player_input(1, 1), GameState::InProgress);
        assert_eq!(game.player_input(1, 0), GameState::InProgress);
        assert_eq!(game.player_input(2, 2), GameState::Win);
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
                "win" => GameState::Win,
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
            game.board_size = board_size;
            println!("{}", &game.board_size);
            assert_eq!(game.check_game_state(), state);
        }
    }
}
