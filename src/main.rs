use game_logic::Game;
use iced::{Align, Application, Button, Column, Command, Element, HorizontalAlignment, Length, Row, Settings, Text, VerticalAlignment, button, executor, window};
mod game_logic;

const WINDOW_SIZE_X: u32 = 500;
const WINDOW_SIZE_Y: u32 = 600;

const GRID_BUTTON_SPACING: u16 = 12;

fn main() {
    TicTacToe::run(Settings {
        window: window::Settings {
            size: (WINDOW_SIZE_X, WINDOW_SIZE_Y),
            min_size: Some((WINDOW_SIZE_X, WINDOW_SIZE_Y)),
            ..window::Settings::default()
        },
        antialiasing: true,
        ..Settings::default()
    }).expect("Failed to start the application");
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

struct TicTacToe {
    board_size: u8,
    game: Game,
    system_text: String,
    state: AppState,
    board: Vec<Vec<button::State>>,
}

impl Application for TicTacToe {
    type Executor = executor::Default;

    type Message = Message;

    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (
            TicTacToe {
                board_size: 3,
                state: AppState::GameOver {
                    new_3_game_button: button::State::new(),
                    new_4_game_button: button::State::new(),
                    new_5_game_button: button::State::new(),
                },
                game: Game::new(),
                system_text: String::from("Select board to start the game"),
                board: Vec::new(),
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
                if self.game.board.get(&(x, y)).unwrap() != &' '
                    || matches!(self.state, AppState::GameOver { new_3_game_button: _, new_4_game_button: _, new_5_game_button: _ }){
                    return Command::none();
                }
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
                        text.push_str(" won!");
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

                self.board.clear();
                for _i in 0..s {
                    let mut row = Vec::new();
                    for _j in 0..s {
                        row.push(button::State::new());

                    }
                    self.board.push(row);
                }

                let winner: char = self.game.get_next_player_symbol();
                let mut text: String = String::from(winner.to_string());
                text.push_str(" turn!");
                self.system_text = text;
                self.state = AppState::Playing {
                    reset_button: button::State::new(),
                }
            },
            Message::Reset => {
                self.system_text = String::from("Game Reset. Choose a new board");

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
        let system_label = Text::new(&self.system_text)
            .horizontal_alignment(HorizontalAlignment::Center)
            .vertical_alignment(VerticalAlignment::Center)
            .size(36);

        let owners = &self.game.board;

        let board = self.board.iter_mut()
            .enumerate()
            .fold(Row::new(), |row, (x, v)| {
                row.push(
                    v.iter_mut()
                     .enumerate()
                     .fold(Column::new(), |clm, (y, btn)| {
                         clm.push(
                             Button::new(btn,
                                         Text::new(owners.get(&(x as u8, y as u8)).unwrap().to_string())
                                         .vertical_alignment(VerticalAlignment::Center)
                                         .horizontal_alignment(HorizontalAlignment::Center)
                                         .size(64)
                             ).on_press(Message::PlaceSymbol(x as u8, y as u8))
                                .height(Length::Units(72))
                                .width(Length::Units(72))
                        ).spacing(GRID_BUTTON_SPACING)
                     })
                ).spacing(GRID_BUTTON_SPACING)
            });

        match &mut self.state {
            AppState::Playing { reset_button } => {
                Column::new()
                    .push(
                        Row::new().push(
                            Button::new(reset_button,
                                        Text::new("Reset")
                                        .horizontal_alignment(HorizontalAlignment::Center)
                                        .size(32)
                            )
                                .on_press(Message::Reset)
                        ).padding(16)
                    )
                    .push(system_label)
                    .push(board)
                    .spacing(16)
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .align_items(Align::Center)
                    .into()
            },
            AppState::GameOver { new_3_game_button, new_4_game_button, new_5_game_button } => {
                Column::new()
                    .push(
                        Row::new().push(
                            Button::new(new_3_game_button,
                                        Text::new("3x3")
                                        .horizontal_alignment(HorizontalAlignment::Center)
                                        .size(32)
                            )
                                .on_press(Message::CreateGame(3))
                        )
                        .push(
                            Button::new(new_4_game_button,
                                        Text::new("4x4")
                                        .horizontal_alignment(HorizontalAlignment::Center)
                                        .size(32)
                            )
                                .on_press(Message::CreateGame(4))
                        )
                        .push(
                            Button::new(new_5_game_button,
                                        Text::new("5x5")
                                        .horizontal_alignment(HorizontalAlignment::Center)
                                        .size(32)
                            )
                                .on_press(Message::CreateGame(5))
                        ).padding(16)
                            .spacing(GRID_BUTTON_SPACING)
                    )
                    .push(system_label)
                    .push(board)
                    .spacing(16)
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .align_items(Align::Center)
                    .into()
            },
        }
    }
}
