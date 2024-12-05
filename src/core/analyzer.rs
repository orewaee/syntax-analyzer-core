use crate::core::state::State;

const DIGITS: Vec<char> = "0123456789".chars().collect();
const LETTERS: Vec<char> = "abcdefghijklmnopqrstuvwxyz".chars().collect();

pub fn analyze(chain: &str, terminal: char) -> Result<(), (usize, &str)> {
    let chars = chain
        .to_ascii_lowercase()
        .chars().collect::<Vec<char>>();

    let mut state = State::Start;
    let mut index = 0;
    let mut symbol: char;

    while index < chain.len() && state != State::Start && state != State::Error {
        symbol = chars[index];

        match state {
            State::Start => match symbol {
                'f' => state = State::ForF,

                _ => {
                    state = State::Error;
                    return Err((index, "maybe you want to use \"f\""));
                }
            }

            State::ForF => match symbol {
                'o' => state = State::ForO,

                _ => {
                    state = State::Error;
                    return Err((index, "maybe you want to use \"o\""));
                }
            }

            State::ForO => match symbol {
                'r' => state = State::ForR,

                _ => {
                    state = State::Error;
                    return Err((index, "maybe you want to use \"r\""));
                }
            }

            State::ForR => match symbol {
                ' ' => state = State::ForSpaces,

                _ => {
                    state = State::Error;
                    return Err((index, "maybe you want to use \"r\""));
                }
            }

            State::ForSpaces => {
                if symbol == ' ' {
                    state = State::ForSpaces;
                    index += 1;
                    continue;
                }

                if LETTERS.contains(&symbol) {
                    state = State::IdLetter;
                    index += 1;
                    continue;
                }

                state = State::Error;
                return Err((index, "error"));
            }

            State::IdLetter | State::IdLetterOrDigit => {
                if symbol == ' ' {
                    state = State::IdSpaces;
                    index += 1;
                    continue;
                }

                if symbol == ':' {
                    state = State::Colon;
                    index += 1;
                    continue;
                }

                if symbol == '[' {
                    state = State::LeftBracket;
                    index += 1;
                    continue;
                }

                if LETTERS.contains(&symbol) || DIGITS.contains(&symbol) {
                    state = State::IdLetterOrDigit;
                    index += 1;
                    continue;
                }
            }

            State::IdSpaces => match symbol {
                ' ' => state = State::IdSpaces,
                ':' => state = State::Colon,
                '[' => state = State::LeftBracket,

                _ => {
                    state = State::Error;
                    return Err((index, "error"));
                }
            }
        }
    }

    if state != State::Finish {
        return Err((index, "use end terminal for close chain"));
    }

    Ok(())
}
