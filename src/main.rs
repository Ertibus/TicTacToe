
use fltk::{app, button::Button, prelude::*, window::Window};
mod game_logic;

const WINDOW_SIZE_X: i32 = 400;
const WINDOW_SIZE_Y: i32 = 600;

fn main() {
    let app = app::App::default();
    let mut wind = Window::default().with_size(WINDOW_SIZE_X, WINDOW_SIZE_Y).with_label("PD2 TicTacToe");
    //let mut frame = Frame::default().with_size(200, 100).center_of(&wind);

    let board_size = 3;


    let board_offset_x: i32 = ((WINDOW_SIZE_X as f32) / 2.0 - 72.0 * (board_size as f32) / 2.0).ceil() as i32;
    let board_offset_y: i32 = ((WINDOW_SIZE_Y as f32) / 2.0 - 72.0 * (board_size as f32) / 2.0).ceil() as i32;

    for x in 0..board_size {
        for y in 0..board_size {
            let mut but = Button::new(board_offset_x + 72 * x, board_offset_y + 72 * y, 64, 64, "0");
            but.set_callback(|b| b.set_label("X"));
        }
    }

    wind.end();
    wind.show();

    app.run().unwrap();
}
