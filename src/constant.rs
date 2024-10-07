#[derive(Debug, Eq, PartialEq)]
enum State {
    Start,
    One,
    Two,
    ThreeToNine,
    ZeroToFour,
    Five,
    ZeroToNine,
    Space,
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
                '1' => state = State::One,
                '2' => state = State::Two,
                '3' | '4' | '5' | '6' | '7' | '8' | '9' => state = State::ThreeToNine,

                _ => {
                    state = State::Error;
                    println!("error. index = {}", index);
                    break;
                }
            }

            State::One => match symbol {
                ' ' => state = State::Finish,
                '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => state = State::ZeroToNine,

                _ => {
                    state = State::Error;
                    println!("error. index = {}", index);
                    break;
                }
            }

            State::ZeroToNine => match symbol {
                ' ' => state = State::Finish,
                '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => state = State::Space,

                _ => {
                    state = State::Error;
                    println!("error. index = {}", index);
                    break;
                }
            }

            State::Two => match symbol {
                ' ' => state = State::Finish,
                '0' | '1' | '2' | '3' | '4' => state = State::ZeroToFour,
                '5' => state = State::Five,
                '6' | '7' | '8' | '9' => state = State::Space,

                _ => {
                    state = State::Error;
                    println!("error. index = {}", index);
                    break;
                }
            }

            State::ZeroToFour => match symbol {
                ' ' => state = State::Finish,
                '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => state = State::Space,

                _ => {
                    state = State::Error;
                    println!("error. index = {}", index);
                    break;
                }
            }

            State::Five => match symbol {
                ' ' => state = State::Finish,
                '0' | '1' | '2' | '3' | '4' | '5' | '6' => state = State::Space,

                _ => {
                    state = State::Error;
                    println!("error. index = {}", index);
                    break;
                }
            }

            State::ThreeToNine => match symbol {
                ' ' => state = State::Finish,
                '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => state = State::Space,

                _ => {
                    state = State::Error;
                    println!("error. index = {}", index);
                    break;
                }
            }

            State::Space => match symbol {
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