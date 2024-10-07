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

pub fn is_constant(string: &str) -> Result<(), (usize, &str)> {
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
                    return Err((index, "the constant must start with a number between 1 and 9"));
                }
            }

            State::One | State::ZeroToNine | State::ZeroToFour | State::ThreeToNine => match symbol {
                ' ' => state = State::Finish,
                '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => state = State::ZeroToNine,

                _ => {
                    state = State::Error;
                    return Err((index, "there must be a space or a number between 0 and 9"));
                }
            }

            State::Two => match symbol {
                ' ' => state = State::Finish,
                '0' | '1' | '2' | '3' | '4' => state = State::ZeroToFour,
                '5' => state = State::Five,
                '6' | '7' | '8' | '9' => state = State::Space,

                _ => {
                    state = State::Error;
                    return Err((index, "there must be a space or a number between 0 and 9"));
                }
            }

            State::Five => match symbol {
                ' ' => state = State::Finish,
                '0' | '1' | '2' | '3' | '4' | '5' | '6' => state = State::Space,

                _ => {
                    state = State::Error;
                    return Err((index, "there must be a space or a number between 0 and 6"));
                }
            }

            State::Space => match symbol {
                ' ' => state = State::Finish,

                _ => {
                    state = State::Error;
                    return Err((index, "there should be a space here"));
                }
            }

            _ => {
                state = State::Error;
                return Err((index, "unexpected error"));
            }
        }

        index += 1;
    }

    if state != State::Finish {
        return Err((index, "there should be a space here"));
    }

    Ok(())
}