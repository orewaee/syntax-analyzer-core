#[derive(Debug, Eq, PartialEq)]
enum State {
    Start,
    A, B, C, D, E, F,
    G, H, I, J, K,
    Error,
    Finish,
}

pub fn is_constant(string: &str) {
    let mut state = State::Start;
    let mut index = 0;
    let mut symbol: char;

    while state != State::Finish && state != State::Error && index < string.len() {
        symbol = string.chars().nth(index).unwrap();
        match state {
            State::Start => match symbol {
                '1' => state = State::A,
                '2' => state = State::F,
                '3' | '4' | '5' | '6' | '7' | '8' | '9' => state = State::D,

                _ => {
                    state = State::Error;
                    println!("error. index = {}", index);
                    break;
                }
            }

            State::A => match symbol {
                ' ' => state = State::Finish,
                '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => state = State::B,

                _ => {
                    state = State::Error;
                    println!("error. index = {}", index);
                    break;
                }
            }

            State::B => match symbol {
                ' ' => state = State::Finish,
                '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => state = State::C,

                _ => {
                    state = State::Error;
                    println!("error. index = {}", index);
                    break;
                }
            }

            State::C => match symbol {
                ' ' => state = State::Finish,

                _ => {
                    state = State::Error;
                    println!("error. index = {}", index);
                    break;
                }
            }

            State::D => match symbol {
                ' ' => state = State::Finish,
                '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => state = State::E,

                _ => {
                    state = State::Error;
                    println!("error. index = {}", index);
                    break;
                }
            }

            State::F => match symbol {
                ' ' => state = State::Finish,
                '0' | '1' | '2' | '3' | '4' => state = State::G,
                '5' => state = State::H,
                '6' | '7' | '8' | '9' => state = State::I,

                _ => {
                    state = State::Error;
                    println!("error. index = {}", index);
                    break;
                }
            }

            State::G => match symbol {
                ' ' => state = State::Finish,
                '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => state = State::J,

                _ => {
                    state = State::Error;
                    println!("error. index = {}", index);
                    break;
                }
            }

            State::J => match symbol {
                ' ' => state = State::Finish,

                _ => {
                    state = State::Error;
                    println!("error. index = {}", index);
                    break;
                }
            }

            State::H => match symbol {
                ' ' => state = State::Finish,
                '0' | '1' | '2' | '3' | '4' | '5' | '6' => state = State::K,

                _ => {
                    state = State::Error;
                    println!("error. index = {}", index);
                    break;
                }
            }

            State::K => match symbol {
                ' ' => state = State::Finish,

                _ => {
                    state = State::Error;
                    println!("error. index = {}", index);
                    break;
                }
            }

            State::I => match symbol {
                ' ' => state = State::Finish,

                _ => {
                    state = State::Error;
                    println!("error. index = {}", index);
                    break;
                }
            }

            _ => {
                state = State::Error;
                println!("error. index = {}", index);
                break;
            }
        }

        index += 1;
    }

    if state == State::Finish {
        println!("success");
    }
}