mod error;
mod state;
mod constant;

use crate::constant::is_constant;

fn main() {
    is_constant("39 ");
    is_constant("25 ");

    /*
    let string = "xxxyyasdz;";

    let mut state = State::S;
    let mut index = 0;
    let mut symbol: char;

    let mut x = 0;
    let mut y = 0;
    let mut z = 0;

    while state != State::F && state != State::E && index < string.len() {
        symbol = string.chars().nth(index).unwrap();
        match state {
            State::S => {
                if symbol == 'x' {
                    state = State::A;
                    x += 1;
                } else {
                    state = State::E;
                    with_message(string, index, "maybe you wanted to start with x");
                    break;
                }
            }

            State::A => {
                if symbol == 'x' {
                    state = State::A;
                    x += 1;
                } else if symbol == 'y' {
                    state = State::B;
                    y += 1;
                } else {
                    state = State::E;
                    with_message(string, index, "maybe you wanted to use y");
                    break;
                }
            }

            State::B => {
                if symbol == 'y' {
                    state = State::B;
                    y += 1;
                } else if symbol == 'z' {
                    state = State::C;
                    z += 1;
                } else {
                    state = State::E;
                    with_message(string, index, "maybe you wanted to use z");
                    break;
                }
            }

            State::C => {
                if symbol == 'z' {
                    state = State::C;
                    z += 1;
                } else if symbol == ';' {
                    state = State::F;
                } else {
                    state = State::E;
                    with_message(string, index, "maybe you wanted to use ;");
                    break;
                }
            }

            State::E | State::F => {
                break;
            }
        }

        index += 1;
    }

    if state == State::F {
        println!("success");
        println!("x: {}; y: {}; z: {};", x, y, z)
    }
    */
}
