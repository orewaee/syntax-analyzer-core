use crate::core::state::State;
use crate::core::constants::{LETTERS, DIGITS};

pub fn analyze(chain: &str, terminal: char) -> Result<(), (usize, &str)> {
    let chars = chain
        .to_ascii_lowercase()
        .chars().collect::<Vec<char>>();

    let mut state = State::Start;
    let mut index = 0;
    let mut symbol: char;

    while index < chain.len() && state != State::Finish && state != State::Error {
        symbol = chars[index];
        println!("symbol = '{symbol}'; state = {state:?}");

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
                    state = State::Id;
                    index += 1;
                    continue;
                }

                state = State::Error;
                return Err((index, "error"));
            }

            State::Id => {
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
                    state = State::Id;
                    index += 1;
                    continue;
                }

                state = State::Error;
                return Err((index, "error"));
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

            State::LeftBracket => {
                if symbol == ' ' {
                    state = State::LeftBracket;
                    index += 1;
                    continue;
                }

                if LETTERS.contains(&symbol) {
                    state = State::ListId;
                    index += 1;
                    continue;
                }

                if DIGITS[1..].contains(&symbol) {
                    state = State::ListConst;
                    index += 1;
                    continue;
                }

                state = State::Error;
                return Err((index, "error"));
            }

            State::ListId => {
                if symbol == ' ' {
                    state = State::ListSpaces;
                    index += 1;
                    continue;
                }

                if symbol == ',' {
                    state = State::LeftBracket;
                    index += 1;
                    continue;
                }

                if symbol == ']' {
                    state = State::RightBracket;
                    index += 1;
                    continue;
                }

                if LETTERS.contains(&symbol) || DIGITS.contains(&symbol) {
                    state = State::ListId;
                    index += 1;
                    continue;
                }

                state = State::Error;
                return Err((index, "error"));
            }

            State::ListConst => {
                if symbol == ' ' {
                    state = State::ListSpaces;
                    index += 1;
                    continue;
                }

                if symbol == ',' {
                    state = State::LeftBracket;
                    index += 1;
                    continue;
                }

                if symbol == ']' {
                    state = State::RightBracket;
                    index += 1;
                    continue;
                }

                if DIGITS.contains(&symbol) {
                    state = State::ListConst;
                    index += 1;
                    continue;
                }

                state = State::Error;
                return Err((index, "error"));
            }

            State::ListSpaces => match symbol {
                ' ' => state = State::ListSpaces,
                ',' => state = State::LeftBracket,
                ']' => state = State::RightBracket,

                _ => {
                    state = State::Error;
                    return Err((index, "error"));
                }
            }

            State::RightBracket => match symbol {
                ' ' => state = State::RightBracket,
                ':' => state = State::Colon,

                _ => {
                    state = State::Error;
                    return Err((index, "error"));
                }
            }

            State::Colon => match symbol {
                '=' => state = State::Equal,

                _ => {
                    state = State::Error;
                    return Err((index, "error"));
                }
            }

            State::Equal => {
                if symbol == ' ' {
                    state = State::Equal;
                    index += 1;
                    continue;
                }

                if symbol = '0' {
                    state = State::ConstZero;
                    index += 1;
                    continue;
                }

                if symbol == '-' {
                    state = State::ConstSign;
                    index += 1;
                    continue;
                }

                if DIGITS[1..].contains(&symbol) {
                    state = State::Const;
                    index += 1;
                    continue;
                }

                state = State::Error;
                return Err((index, "error"));
            }

            State::ConstSign => {
                if DIGITS[1..].contains(&symbol) {
                    state = State::Const;
                    index += 1;
                    continue;
                }

                state = State::Error;
                return Err((index, "error"));
            }

            State::Const => {
                if symbol == ' ' {
                    state = State::ConstSpaces;
                    index += 1;
                    continue;
                }

                if DIGITS.contains(&symbol) {
                    state = State::Const;
                    index += 1;
                    continue;
                }

                state = State::Error;
                return Err((index, "error"));
            }

            State::ConstZero => match symbol {
                ' ' => state = State::ConstSpaces,

                _ => {
                    state = State::Error;
                    return Err((index, "error"));
                }
            }

            State::ConstSpaces => match symbol {
                ' ' => state = State::ConstSpaces,
                't' => state = State::ToT,

                _ => {
                    state = State::Error;
                    return Err((index, "error"));
                }
            }

            State::ToT => match symbol {
                'o' => state = State::ToO,

                _ => {
                    state = State::Error;
                    return Err((index, "error"));
                }
            }

            _ => {
                state = State::Error;
                return Err((index, "error"));
            }
        }

        index += 1;
    }

    if state != State::Finish {
        return Err((index, "use end terminal for close chain"));
    }

    Ok(())
}
