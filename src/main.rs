mod color;

use color::{green, red};

#[derive(Debug, PartialEq, Eq)]
enum State {
    S,
    A,
    B,
    C,
    F,
    E,
}

pub fn error(string: &str, index: usize, message: &str) {
    let rigth = &string[..index];
    let wrong = &string[index..(index + 1)];
    let other = &string[(index + 1)..];

    println!("{}{}{}", green(rigth), red(wrong), other);
    println!("{}{}", " ".repeat(index), red("^"));

    let error = format!("error: {}", message);
    println!("{}", red(error.as_str()));
}

fn main() {
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
                    error(string, index, "maybe you wanted to start with x");
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
                    error(string, index, "maybe you wanted to use y");
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
                    error(string, index, "maybe you wanted to use z");
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
                    error(string, index, "maybe you wanted to use ;");
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
}
