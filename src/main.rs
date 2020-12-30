//! Push2Pong is the Pong game implemented in Rust for the Ableton Push2.
//!
//! Ableton Push2 is a MIDI instrument with a 960x160 RGB LCD display.
//! Push2 is a USB composite device with a MIDI interface and a generic bulk data interface used to drive the display.
//!
//! ```bash
//! git clone https://github.com/mbracher/push2_pong
//! cd push2_display
//! cargo install push2_pong
//!
//! push2_pong
//! ```
//! or
//! ```bash
//! git clone https://github.com/mbracher/push2_pong
//! cd push2_display
//! cargo run
//! ```
//!
//! ![Photo of Pong on Push2 device](https://raw.githubusercontent.com/mbracher/push2_display/master/doc/assets/push2hello.jpg)
//!
//! # References
//! [Ableton Push Interface](https://github.com/Ableton/push-interface)
//!
//! [Embedded graphics](https://github.com/embedded-graphics/embedded-graphics)

mod score;
mod player;
mod ball;

use crate::score::Score;
use crate::player::Player;
use crate::ball::Ball;

use anyhow::Result;
use thiserror::Error;

use midir::{MidiInput, MidiOutput, Ignore, MidiInputPort, ConnectError, InitError, PortInfoError, SendError};
// use wayang::{Wayang, WayangError};
use push2_display::*;
use gameloop::{GameLoop, FrameAction, GameLoopError};
use std::sync::mpsc::{Receiver};

use embedded_graphics::{
    prelude::*,
    geometry::Point,
    pixelcolor::Bgr565,
};
use std::convert::Infallible;


fn main() {
    match run() {
        Ok(_) => (),
        Err(err) => println!("Error: {}", err)
    }
}

#[derive(Error, Debug)]
pub enum MyError {
    #[error("Ableton Push 2 Midi In not found")]
    NoMidiInFound,

    #[error("Ableton Push 2 Midi Out not found")]
    NoMidiOutFound,

    /// Represents all other cases of `std::io::Error`.
    #[error(transparent)]
    IOError(#[from] std::io::Error),

    #[error(transparent)]
    GameLoopE(#[from] GameLoopError),


    #[error(transparent)]
    MidirError(#[from] ConnectError<MidiInput>),


    #[error(transparent)]
    MidirError2(#[from] InitError),

    // #[error(transparent)]
    // Push2Error(#[from] WayangError),

    #[error(transparent)]
    Push2Error(#[from] Push2DisplayError),

    #[error(transparent)]
    MidiError3(#[from] PortInfoError),

    #[error(transparent)]
    MidirError4(#[from] ConnectError<MidiOutput>),

    #[error(transparent)]
    MidirError5(#[from] SendError),

    #[error(transparent)]
    Infallible(#[from] Infallible),

    #[error(transparent)]
    Other(#[from] anyhow::Error),  // source and Display delegate to anyhow::Error

}

enum MidiMessage {
    X(i8),
    Y(i8),
    X2(i8),
    Y2(i8),
}
fn run() -> Result<(), MyError> {
    let mut midi_in = MidiInput::new("midir reading input")?;
    midi_in.ignore(Ignore::None);
    let in_port = get_midi_in_port(&midi_in, "Ableton Push 2 Live Port")?;

    // let midi_out = MidiOutput::new("My Test Output")?;
    // let out_port= get_midi_out_port(&midi_out, "Ableton Push 2 Live Port")?;
    // let in_port_name = midi_in.port_name(&in_port)?;
    // let mut conn_out = midi_out.connect(&out_port, "midir-test")?;
    // conn_out.send(&[144, 99, 127])?;

    let mut display = Push2Display::new()?;

    use std::sync::mpsc::channel;

    let (tx1, rx1) = channel::<MidiMessage>();
    // _conn_in needs to be a named parameter, because it needs to be kept alive until the end of the scope
    let _conn_in = midi_in.connect(
        &in_port,
        "midir-read-input",
        move |_, message, tx| {
            match message {
                [0xB0, 0x4F, value] => {
                    tx.send(MidiMessage::X(get_endcoder_value(value))).unwrap();
                },
                [0xB0, 0x4E, value] => {
                    tx.send(MidiMessage::Y(get_endcoder_value(value))).unwrap();
                },
                [0xB0, 0x0F, value] => {
                    tx.send(MidiMessage::X2(get_endcoder_value(value))).unwrap();
                },
                [0xB0, 0x47, value] => {
                    tx.send(MidiMessage::Y2(get_endcoder_value(value))).unwrap();
                },
                [0xFE, _] => {

                },
                _ => {
                    // println!("{}: {:X?} (len = {})", stamp, message, message.len());
                },
            }

        },
        tx1
    )?;

    let mut player1 = Player::new(display.size().width as i32 - 51, display.size().height as i32 / 2 , 5, 25 );
    let mut player2 = Player::new(50, display.size().height as i32 / 2, 5, 25 );
    let mut ball = Ball::new(display.size().width as i32 / 2, display.size().height as i32 / 2, 9 );
    ball.speed_x = 3;

    let mut touching : bool = false;
    let mut player1_old_pos = player1.position;
    let mut player2_old_pos = player2.position;

    let mut score = Score::new(5);

    let game_loop = GameLoop::new(60,5)?;
    loop {
        handle_paddles(&rx1, &mut player1.position, &mut player2.position);

        for action in game_loop.actions() {
            match action {
                FrameAction::Tick => {
                    let player1_speed_y = player1.position.y  - player1_old_pos.y;
                    let player2_speed_y = player2.position.y  - player2_old_pos.y;

                    let ball_old_pos = ball.position;

                    ball.position.x = add_max(ball.position.x, ball.speed_x , display.size().width as i32);
                    ball.position.y = add_max(ball.position.y, ball.speed_y, display.size().height as i32);

                    if hit_player(ball.position, ball_old_pos, player1) {
                        if !touching {
                            touching = true;
                            ball.speed_y += player1_speed_y;
                             ball.speed_x *= -1;
                        }
                    }
                    else if hit_player(ball.position, ball_old_pos, player2) {
                        if !touching {
                            touching = true;
                            ball.speed_y += player2_speed_y;
                            ball.speed_x *= -1;
                        }
                    }
                    else {
                        touching = false;
                    }

                    if ball.position.y >= display.size().height as i32 - 1 || ball.position.y <= 0 {
                        ball.speed_y *= -1;
                    }

                    if ball.position.x >= display.size().width as i32 - 1 {
                        ball.position.x = display.size().width as i32 / 2;
                        ball.position.y = display.size().height as i32 / 2;
                        ball.speed_x *= -1;
                        ball.speed_y = 0;

                        score.player(1)
                    }
                    else if ball.position.x <= 0  {
                        ball.position.x = display.size().width as i32 / 2;
                        ball.position.y = display.size().height as i32 / 2;
                        ball.speed_x *= -1;
                        ball.speed_y = 0;

                        score.player(2);
                    }

                    match score.winner() {
                        1 => {
                            score.reset();
                        },
                        2 => {
                            score.reset();
                        },
                        _ => {},
                    }
                    player1_old_pos = player1.position;
                    player2_old_pos = player2.position;
                },

                FrameAction::Render { interpolation: _interpolation } => {
                    display.clear(Bgr565::BLACK)?;

                    player1.draw(&mut display)?;
                    player2.draw(&mut display)?;
                    ball.draw(&mut display)?;
                    score.draw(&mut display)?;

                    display.flush()?;
                },
            }
        }
    }
}

fn line_collision(x1: f32, y1: f32, x2: f32, y2: f32, x3: f32, y3: f32, x4: f32, y4: f32) -> bool {
    //http://www.jeffreythompson.org/collision-detection/line-line.php
    let rc = (y4-y3)*(x2-x1) - (x4-x3)*(y2-y1);
    let u_a = ((x4-x3)*(y1-y3) - (y4-y3)*(x1-x3)) / rc;
    let u_b = ((x2-x1)*(y1-y3) - (y2-y1)*(x1-x3)) / rc;

    u_a >= 0. && u_a <= 1.0 && u_b >= 0.0 && u_b <= 1.0
}

fn hit_player(ball_pos: Point, ball_old_pos: Point, player: Player) -> bool {
    line_collision(
        ball_old_pos.x as f32, ball_old_pos.y as f32,
        ball_pos.x as f32, ball_pos.y as f32,
        player.position.x as f32, player.position.y as f32 , // - player.size.height as f32 / 2.0
        player.position.x as f32, player.position.y as f32 + player.size.height as f32 ) //  / 2.0
}

fn add_max(c: i32, v: i32, max: i32) -> i32 {
    let x = c  + v;
    if x < 0 {
        0
    } else if x >= max {
        max - 1
    } else {
        x
    }
}

fn handle_paddles(rx1: &Receiver<MidiMessage>, position: &mut Point, position2: &mut Point)  {
    loop {
        match rx1.try_recv() {
            Ok(MidiMessage::X(value)) => {
                position.x = add_max(position.x, value as i32, DISPLAY_WIDTH as i32)
            },
            Ok(MidiMessage::Y(value)) => {
                position.y = add_max(position.y, value as i32, DISPLAY_HEIGHT as i32)
            },
            Ok(MidiMessage::X2(value)) => {
                position2.x = add_max(position2.x, value as i32, DISPLAY_WIDTH as i32)
            },
            Ok(MidiMessage::Y2(value)) => {
                position2.y = add_max(position2.y, value as i32, DISPLAY_HEIGHT as i32)
            },
            _ => {
                break;
            },
        }
    }
}

fn get_endcoder_value(value: &u8) -> i8 {
    let is_right: bool = (value & 0xC0) == 0;
    if is_right {
        (value & 0x3F) as i8
    } else {
        (64 - ((value & 0x3F) as i8)) * -1
    }
}

fn get_midi_in_port(midi_in: &MidiInput, port_name: &str) -> Result<MidiInputPort, MyError> {
    // Get an input port (read from console if multiple are available)
    let in_ports = midi_in.ports();
    let ip = in_ports.iter().find(|&x| midi_in.port_name(x).unwrap_or_default() == port_name.to_string()).ok_or(MyError::NoMidiInFound)?;
    Ok(ip.clone())
}

// fn get_midi_out_port(midi_out: &MidiOutput, port_name: &str) -> Result<MidiOutputPort, MyError> {
//     let out_ports = midi_out.ports();
//     let p = out_ports.iter().find(|&x| midi_out.port_name(x).unwrap_or_default() == port_name.to_string()).ok_or(MyError::NoMidiOutFound)?;
//     Ok(p.clone())
// }


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn hit_player1() {
        let player = Player::new(50, 50, 5, 15);
        let mut ball = Ball::new(45, 50, 3);
        let old_pos = Point::new(55, 50);
        ball.speed_x = 3;

        assert_eq!(hit_player(ball.position, old_pos, player), true);
    }

    #[test]
    fn hit_player2() {
        let player = Player::new(50, 50, 5, 15);
        let mut ball = Ball::new(65, 50, 3);
        let old_pos = Point::new(55, 50);
        ball.speed_x = 3;

        assert_eq!(hit_player(ball.position, old_pos, player), false);
    }

    #[test]
    fn hit_player3() {
        let player = Player::new(50, 50, 5, 15);
        let mut ball = Ball::new(45, 50, 3);
        let old_pos = Point::new(55, 50);
        ball.speed_x = -3;

        assert_eq!(hit_player(ball.position, old_pos, player), true);
    }

}
