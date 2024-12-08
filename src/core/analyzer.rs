use crate::core::state::State;
use crate::core::constants::{LETTERS, DIGITS};

use crate::semantics::id::IdSemantics;
use crate::semantics::unsigned_const::UnsignedConstSemantics;

pub fn analyze(chain: &str, terminal: char) -> Result<(), (usize, &str)> {
    let chars = chain
        .to_ascii_lowercase()
        .chars().collect::<Vec<char>>();

    let mut state = State::Start;
    let mut index = 0;
    let mut symbol: char;

    let mut id_semantics = IdSemantics::new();
    let mut unsigned_const_semantics = UnsignedConstSemantics::new();

    while index < chain.len() && state != State::Finish && state != State::Error {
        symbol = chars[index];
        // println!("symbol = '{symbol}'; state = {state:?}");

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
                    id_semantics.push(symbol);

                    state = State::Id;
                    index += 1;
                    continue;
                }

                state = State::Error;
                return Err((index, "error"));
            }

            State::Id => {
                if symbol == ' ' {
                    if !id_semantics.valid_length() {
                        state = State::Error;
                        return Err((index, "id length should be from 1 to 8 chars"));
                    }

                    if id_semantics.has_keyword() {
                        state = State::Error;
                        return Err((index, "id should not include keywords"));
                    }

                    if id_semantics.already_exists() {
                        state = State::Error;
                        return Err((index, "ids must not be repeated"));
                    }

                    id_semantics.save();

                    state = State::IdSpaces;
                    index += 1;
                    continue;
                }

                if symbol == ':' {
                    if !id_semantics.valid_length() {
                        state = State::Error;
                        return Err((index, "id length should be from 1 to 8 chars"));
                    }

                    if id_semantics.has_keyword() {
                        state = State::Error;
                        return Err((index, "id should not include keywords"));
                    }

                    if id_semantics.already_exists() {
                        state = State::Error;
                        return Err((index, "ids must not be repeated"));
                    }

                    id_semantics.save();

                    state = State::Colon;
                    index += 1;
                    continue;
                }

                if symbol == '[' {
                    if !id_semantics.valid_length() {
                        state = State::Error;
                        return Err((index, "id length should be from 1 to 8 chars"));
                    }

                    if id_semantics.has_keyword() {
                        state = State::Error;
                        return Err((index, "id should not include keywords"));
                    }

                    if id_semantics.already_exists() {
                        state = State::Error;
                        return Err((index, "ids must not be repeated"));
                    }

                    id_semantics.save();

                    state = State::LeftBracket;
                    index += 1;
                    continue;
                }

                if LETTERS.contains(&symbol) || DIGITS.contains(&symbol) {
                    id_semantics.push(symbol);

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
                    id_semantics.push(symbol);

                    state = State::ListId;
                    index += 1;
                    continue;
                }

                if DIGITS[1..].contains(&symbol) {
                    unsigned_const_semantics.push(symbol);

                    state = State::ListConst;
                    index += 1;
                    continue;
                }

                state = State::Error;
                return Err((index, "error"));
            }

            State::ListId => {
                if symbol == ' ' {
                    if !id_semantics.valid_length() {
                        state = State::Error;
                        return Err((index, "id length should be from 1 to 8 chars"));
                    }

                    if id_semantics.has_keyword() {
                        state = State::Error;
                        return Err((index, "id should not include keywords"));
                    }

                    if id_semantics.already_exists() {
                        state = State::Error;
                        return Err((index, "ids must not be repeated"));
                    }

                    id_semantics.save();

                    state = State::ListSpaces;
                    index += 1;
                    continue;
                }

                if symbol == ',' {
                    if !id_semantics.valid_length() {
                        state = State::Error;
                        return Err((index, "id length should be from 1 to 8 chars"));
                    }

                    if id_semantics.has_keyword() {
                        state = State::Error;
                        return Err((index, "id should not include keywords"));
                    }

                    if id_semantics.already_exists() {
                        state = State::Error;
                        return Err((index, "ids must not be repeated"));
                    }

                    id_semantics.save();

                    state = State::LeftBracket;
                    index += 1;
                    continue;
                }

                if symbol == ']' {
                    if !id_semantics.valid_length() {
                        state = State::Error;
                        return Err((index, "id length should be from 1 to 8 chars"));
                    }

                    if id_semantics.has_keyword() {
                        state = State::Error;
                        return Err((index, "id should not include keywords"));
                    }

                    if id_semantics.already_exists() {
                        state = State::Error;
                        return Err((index, "ids must not be repeated"));
                    }

                    id_semantics.save();

                    state = State::RightBracket;
                    index += 1;
                    continue;
                }

                if LETTERS.contains(&symbol) || DIGITS.contains(&symbol) {
                    id_semantics.push(symbol);

                    state = State::ListId;
                    index += 1;
                    continue;
                }

                state = State::Error;
                return Err((index, "error"));
            }

            State::ListConst => {
                if symbol == ' ' {
                    if !unsigned_const_semantics.valid() {
                        state = State::Error;
                        return Err((index, "constant must be between 1 and 256"));
                    }

                    unsigned_const_semantics.save();

                    state = State::ListSpaces;
                    index += 1;
                    continue;
                }

                if symbol == ',' {
                    if !unsigned_const_semantics.valid() {
                        state = State::Error;
                        return Err((index, "constant must be between 1 and 256"));
                    }

                    unsigned_const_semantics.save();

                    state = State::LeftBracket;
                    index += 1;
                    continue;
                }

                if symbol == ']' {
                    if !unsigned_const_semantics.valid() {
                        state = State::Error;
                        return Err((index, "constant must be between 1 and 256"));
                    }

                    unsigned_const_semantics.save();

                    state = State::RightBracket;
                    index += 1;
                    continue;
                }

                if DIGITS.contains(&symbol) {
                    unsigned_const_semantics.push(symbol);

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

                if symbol == '0' {
                    state = State::StConstZero;
                    index += 1;
                    continue;
                }

                if symbol == '-' {
                    state = State::StConstMinus;
                    index += 1;
                    continue;
                }

                if DIGITS[1..].contains(&symbol) {
                    state = State::StConst;
                    index += 1;
                    continue;
                }

                state = State::Error;
                return Err((index, "error"));
            }

            State::StConstMinus => {
                if DIGITS[1..].contains(&symbol) {
                    state = State::StConst;
                    index += 1;
                    continue;
                }

                state = State::Error;
                return Err((index, "error"));
            }

            State::StConst => {
                if symbol == ' ' {
                    state = State::StSpaces;
                    index += 1;
                    continue;
                }

                if DIGITS.contains(&symbol) {
                    state = State::StConst;
                    index += 1;
                    continue;
                }

                state = State::Error;
                return Err((index, "error"));
            }

            State::StConstZero => match symbol {
                ' ' => state = State::StSpaces,

                _ => {
                    state = State::Error;
                    return Err((index, "error"));
                }
            }

            State::StSpaces => match symbol {
                ' ' => state = State::StSpaces,
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

            State::ToO => match symbol {
                ' ' => state = State::StNdSpaces,

                _ => {
                    state = State::Error;
                    return Err((index, "error"));
                }
            }

            State::StNdSpaces => {
                if symbol == ' ' {
                    state = State::StNdSpaces;
                    index += 1;
                    continue;
                }

                if symbol == '0' {
                    state = State::NdConstZero;
                    index += 1;
                    continue;
                }

                if symbol == '-' {
                    state = State::NdConstMinus;
                    index += 1;
                    continue;
                }

                if DIGITS[1..].contains(&symbol) {
                    state = State::NdConst;
                    index += 1;
                    continue;
                }

                state = State::Error;
                return Err((index, "error"));
            }

            State::NdConstMinus => {
                if DIGITS[1..].contains(&symbol) {
                    state = State::NdConst;
                    index += 1;
                    continue;
                }

                state = State::Error;
                return Err((index, "error"));
            }

            State::NdConst => {
                if symbol == ' ' {
                    state = State::NdSpaces;
                    index += 1;
                    continue;
                }

                if DIGITS.contains(&symbol) {
                    state = State::NdConst;
                    index += 1;
                    continue;
                }

                state = State::Error;
                return Err((index, "error"));
            }

            State::NdConstZero => match symbol {
                ' ' => state = State::NdSpaces,

                _ => {
                    state = State::Error;
                    return Err((index, "error"));
                }
            }

            State::NdSpaces => match symbol {
                ' ' => state = State::NdSpaces,
                'b' => state = State::ByB,
                'd' => state = State::DoD,

                _ => {
                    state = State::Error;
                    return Err((index, "error"));
                }
            }

            State::ByB => match symbol {
                'y' => state = State::ByY,

                _ => {
                    state = State::Error;
                    return Err((index, "error"));
                }
            }

            State::ByY => match symbol {
                ' ' => state = State::NdRdSpaces,

                _ => {
                    state = State::Error;
                    return Err((index, "error"));
                }
            }

            State::NdRdSpaces => {
                if symbol == ' ' {
                    state = State::NdRdSpaces;
                    index += 1;
                    continue;
                }

                if symbol == '0' {
                    state = State::RdConstZero;
                    index += 1;
                    continue;
                }

                if symbol == '-' {
                    state = State::RdConstMinus;
                    index += 1;
                    continue;
                }

                if DIGITS[1..].contains(&symbol) {
                    state = State::RdConst;
                    index += 1;
                    continue;
                }

                state = State::Error;
                return Err((index, "error"));
            }

            State::RdConstMinus => {
                if DIGITS[1..].contains(&symbol) {
                    state = State::RdConst;
                    index += 1;
                    continue;
                }

                state = State::Error;
                return Err((index, "error"));
            }

            State::RdConst => {
                if symbol == ' ' {
                    state = State::RdSpaces;
                    index += 1;
                    continue;
                }

                if DIGITS.contains(&symbol) {
                    state = State::RdConst;
                    index += 1;
                    continue;
                }

                state = State::Error;
                return Err((index, "error"));
            }

            State::RdConstZero => match symbol {
                ' ' => state = State::RdSpaces,

                _ => {
                    state = State::Error;
                    return Err((index, "error"));
                }
            }

            State::RdSpaces => match symbol {
                ' ' => state = State::RdSpaces,
                'd' => state = State::DoD,

                _ => {
                    state = State::Error;
                    return Err((index, "error"));
                }
            }

            State::DoD => match symbol {
                'o' => state = State::DoO,

                _ => {
                    state = State::Error;
                    return Err((index, "error"));
                }
            }

            State::DoO => {
                if symbol == ' ' {
                    state = State::DoO;
                    index += 1;
                    continue;
                }

                if symbol == terminal {
                    state = State::Finish;
                    index += 1;
                    continue;
                }

                state = State::Error;
                return Err((index, "error"));
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

    // println!("{:?}", id_semantics.ids);

    Ok(())
}
