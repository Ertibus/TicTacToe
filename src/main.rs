use std::borrow::{Borrow, BorrowMut};

use game_logic::Game;
use iced::{Align, Application, Button, Column, Command, Element, Executor, Length, Settings, Text, button, executor};
mod game_logic;


const WINDOW_SIZE_X: i32 = 400;
const WINDOW_SIZE_Y: i32 = 600;

fn main() {
    UserInterface::run(Settings::default()).expect("Failed to start the application");
}

enum AppState {
    Playing {
        reset_button: button::State,
    },
    GameOver {
        new_3_game_button: button::State,
        new_4_game_button: button::State,
        new_5_game_button: button::State,
    },
}

impl Default for AppState {
    fn default() -> Self {
        AppState::GameOver {
            new_3_game_button: button::State::new(),
            new_4_game_button: button::State::new(),
            new_5_game_button: button::State::new(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    PlaceSymbol(u8, u8),
    CreateGame(u8),
    Reset,
}

struct UserInterface {
    board_size: u8,
    game: Game,
    system_text: String,
    state: AppState,
}

impl Application for UserInterface {
    type Executor = executor::Default;

    type Message = Message;

    type Flags = ();

    fn new(flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (
            UserInterface {
                board_size: 3,
                state: AppState::GameOver {
                    new_3_game_button: button::State::new(),
                    new_4_game_button: button::State::new(),
                    new_5_game_button: button::State::new(),
                },
                game: Game::new(),
                system_text: String::from("Select board size to start the game"),
            },
            Command::none()
        )
    }

    fn title(&self) -> String {
        String::from("PD2. TicTacToe TDD")
    }

    fn update(&mut self, message: Self::Message, _clipboard: &mut iced::Clipboard,) -> Command<Message> {
        match message {
            Message::PlaceSymbol(x, y) => {
                match self.game.player_input(x, y) {
                    game_logic::GameState::Draw => {
                        self.system_text = String::from("It's a draw! Start a new game");

                        self.state = AppState::GameOver {
                            new_3_game_button: button::State::new(),
                            new_4_game_button: button::State::new(),
                            new_5_game_button: button::State::new(),
                        }
                    },
                    game_logic::GameState::Win => {
                        let winner: char = self.game.get_prev_player_symbol();
                        let mut text: String = String::from(winner.to_string());
                        text.push_str(" won! Start a new game?");
                        self.system_text = text;

                        self.state = AppState::GameOver {
                            new_3_game_button: button::State::new(),
                            new_4_game_button: button::State::new(),
                            new_5_game_button: button::State::new(),
                        }
                    },
                    game_logic::GameState::InProgress => {
                        let winner: char = self.game.get_next_player_symbol();
                        let mut text: String = String::from(winner.to_string());
                        text.push_str(" turn!");
                        self.system_text = text;
                    },
                }
            },
            Message::CreateGame(s) => {
                self.board_size = s;
                self.game.create_board(s);
                let winner: char = self.game.get_next_player_symbol();
                let mut text: String = String::from(winner.to_string());
                text.push_str(" turn!");
                self.system_text = text;
                self.state = AppState::Playing {
                    reset_button: button::State::new(),
                }
            },
            Message::Reset => {
                self.system_text = String::from("Game Reset. Choose a new game");

                self.state = AppState::GameOver {
                    new_3_game_button: button::State::new(),
                    new_4_game_button: button::State::new(),
                    new_5_game_button: button::State::new(),
                }
            },
        }
        Command::none()
    }

    fn view(&mut self) -> Element<Message> {
        let system_label = Text::new(&self.system_text).width(Length::Fill).size(16);
        match &mut self.state {
            AppState::Playing { reset_button } => {
                Column::new()
                    .align_items(Align::Center)
                    .push(
                        Button::new(reset_button, Text::new("Reset"))
                            .on_press(Message::Reset)
                            .width(Length::Fill)
                    )
                    .push(system_label)
                    .into()
            },
            AppState::GameOver { new_3_game_button, new_4_game_button, new_5_game_button } => {
                Column::new()
                    .align_items(Align::Center)
                    .push(
                        Button::new(new_3_game_button, Text::new("3x3"))
                            .on_press(Message::CreateGame(3))
                            .width(Length::Fill)
                    )
                    .push(
                        Button::new(new_4_game_button, Text::new("4x4"))
                            .on_press(Message::CreateGame(4))
                            .width(Length::Fill)
                    )
                    .push(
                        Button::new(new_5_game_button, Text::new("5x5"))
                            .on_press(Message::CreateGame(5))
                            .width(Length::Fill)
                    )
                    .push(system_label)
                    .into()
            },
        }
    }



}

/*
    fn run() {
        let app = app::App::default();
        let mut wind = Window::default().with_size(WINDOW_SIZE_X, WINDOW_SIZE_Y).with_label("PD2 TicTacToe");
        //let mut frame = Frame::default().with_size(200, 100).center_of(&wind);

        let board_size = 3;

        let board_offset_x: i32 = ((WINDOW_SIZE_X as f32) / 2.0 - 72.0 * (board_size as f32) / 2.0).ceil() as i32;
        let board_offset_y: i32 = ((WINDOW_SIZE_Y as f32) / 2.0 - 72.0 * (board_size as f32) / 2.0).ceil() as i32;
        let mut game: Game = Game::new();

        for x in 0..board_size {
            for y in 0..board_size {
                let mut but = Button::new(board_offset_x + 72 * x, board_offset_y + 72 * y, 64, 64, "0");


                but.set_callback(move |b| {
                    let xc = (x as u8).clone();
                    let yc = (y as u8).clone();
                    b.set_label("X");
                });
            }
        }

        wind.end();
        wind.show();

        app.run().unwrap();
    }

    fn on_button_press(x: u8, y: u8) {
    }
*/
