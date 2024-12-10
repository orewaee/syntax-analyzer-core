use crate::core::state::State;
use crate::core::constants::{LETTERS, DIGITS};

use crate::semantics::id::IdSemantics;
use crate::semantics::unsigned_const::UnsignedConstSemantics;
use crate::semantics::signed_const::SignedConstSemantics;

use crate::cli::semantics::semantics_html;

pub fn analyze(chain: &str, terminal: char) -> Result<String, (usize, &str)> {
    println!("analyzing {}... ", chain);

    let chars = chain
        .to_ascii_lowercase()
        .chars().collect::<Vec<char>>();

    let mut state = State::Start;
    let mut index = 0;
    let mut symbol: char;

    let mut id_semantics = IdSemantics::new();
    let mut unsigned_const_semantics = UnsignedConstSemantics::new();
    let mut signed_const_semantics = SignedConstSemantics::new();

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
                return Err((index, "maybe you want to use letter"));
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
                return Err((index, "maybe you want to use \":\", \"[\", a|..|z or 0|..|9"));
            }

            State::IdSpaces => match symbol {
                ' ' => state = State::IdSpaces,
                ':' => state = State::Colon,
                '[' => state = State::LeftBracket,

                _ => {
                    state = State::Error;
                    return Err((index, "maybe you want to use \":\" or \"[\""));
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
                return Err((index, "maybe you want to use \":\", \"[\", a|..|z or 1|..|9"));
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
                return Err((index, "maybe you want to use \",\", \"]\", a|..|z or 0|..|9"));
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
                return Err((index, "maybe you want to use \",\", \"]\" or 0|..|9"));
            }

            State::ListSpaces => match symbol {
                ' ' => state = State::ListSpaces,
                ',' => state = State::LeftBracket,
                ']' => state = State::RightBracket,

                _ => {
                    state = State::Error;
                    return Err((index, "maybe you want to use \",\" or \"]\""));
                }
            }

            State::RightBracket => match symbol {
                ' ' => state = State::RightBracket,
                ':' => state = State::Colon,

                _ => {
                    state = State::Error;
                    return Err((index, "maybe you want to use \":\""));
                }
            }

            State::Colon => match symbol {
                '=' => state = State::Equal,

                _ => {
                    state = State::Error;
                    return Err((index, "maybe you want to use \"=\""));
                }
            }

            State::Equal => {
                if symbol == ' ' {
                    state = State::Equal;
                    index += 1;
                    continue;
                }

                if symbol == '0' {
                    signed_const_semantics.push(symbol);

                    state = State::StConstZero;
                    index += 1;
                    continue;
                }

                if symbol == '-' {
                    signed_const_semantics.push(symbol);

                    state = State::StConstMinus;
                    index += 1;
                    continue;
                }

                if DIGITS[1..].contains(&symbol) {
                    signed_const_semantics.push(symbol);

                    state = State::StConst;
                    index += 1;
                    continue;
                }

                state = State::Error;
                return Err((index, "maybe you want to use \"0\", \"-\" or 1|..|9"));
            }

            State::StConstMinus => {
                if DIGITS[1..].contains(&symbol) {
                    signed_const_semantics.push(symbol);

                    state = State::StConst;
                    index += 1;
                    continue;
                }

                state = State::Error;
                return Err((index, "maybe you want to use 1|..|9"));
            }

            State::StConst => {
                if symbol == ' ' {
                    if !signed_const_semantics.valid() {
                        state = State::Error;
                        return Err((index, "constant must be between -32768 and 32767"));
                    }

                    signed_const_semantics.save();

                    state = State::StSpaces;
                    index += 1;
                    continue;
                }

                if DIGITS.contains(&symbol) {
                    signed_const_semantics.push(symbol);

                    state = State::StConst;
                    index += 1;
                    continue;
                }

                state = State::Error;
                return Err((index, "maybe you want to use 0|..|9"));
            }

            State::StConstZero => match symbol {
                ' ' => {
                    if !signed_const_semantics.valid() {
                        state = State::Error;
                        return Err((index, "constant must be between -32768 and 32767"));
                    }

                    signed_const_semantics.save();

                    state = State::StSpaces
                },

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
                    return Err((index, "maybe you want to use \"t\""));
                }
            }

            State::ToT => match symbol {
                'o' => state = State::ToO,

                _ => {
                    state = State::Error;
                    return Err((index, "maybe you want to use \"o\""));
                }
            }

            State::ToO => match symbol {
                ' ' => state = State::StNdSpaces,

                _ => {
                    state = State::Error;
                    return Err((index, "maybe you want to use space"));
                }
            }

            State::StNdSpaces => {
                if symbol == ' ' {
                    state = State::StNdSpaces;
                    index += 1;
                    continue;
                }

                if symbol == '0' {
                    signed_const_semantics.push(symbol);

                    state = State::NdConstZero;
                    index += 1;
                    continue;
                }

                if symbol == '-' {
                    signed_const_semantics.push(symbol);

                    state = State::NdConstMinus;
                    index += 1;
                    continue;
                }

                if DIGITS[1..].contains(&symbol) {
                    signed_const_semantics.push(symbol);

                    state = State::NdConst;
                    index += 1;
                    continue;
                }

                state = State::Error;
                return Err((index, "maybe you want to use \"0\", \"-\" or 1|..|9"));
            }

            State::NdConstMinus => {
                if DIGITS[1..].contains(&symbol) {
                    signed_const_semantics.push(symbol);

                    state = State::NdConst;
                    index += 1;
                    continue;
                }

                state = State::Error;
                return Err((index, "maybe you want to use 1|..|9"));
            }

            State::NdConst => {
                if symbol == ' ' {
                    if !signed_const_semantics.valid() {
                        state = State::Error;
                        return Err((index, "constant must be between -32768 and 32767"));
                    }

                    signed_const_semantics.save();

                    state = State::NdSpaces;
                    index += 1;
                    continue;
                }

                if DIGITS.contains(&symbol) {
                    signed_const_semantics.push(symbol);

                    state = State::NdConst;
                    index += 1;
                    continue;
                }

                state = State::Error;
                return Err((index, "maybe you want to use 0|..|9"));
            }

            State::NdConstZero => match symbol {
                ' ' => {
                    if !signed_const_semantics.valid() {
                        state = State::Error;
                        return Err((index, "constant must be between -32768 and 32767"));
                    }

                    signed_const_semantics.save();

                    state = State::NdSpaces
                },

                _ => {
                    state = State::Error;
                    return Err((index, "maybe you want to use space"));
                }
            }

            State::NdSpaces => match symbol {
                ' ' => state = State::NdSpaces,
                'b' => state = State::ByB,
                'd' => state = State::DoD,

                _ => {
                    state = State::Error;
                    return Err((index, "maybe you want to use \"b\" or \"d\""));
                }
            }

            State::ByB => match symbol {
                'y' => state = State::ByY,

                _ => {
                    state = State::Error;
                    return Err((index, "maybe you want to use \"y\""));
                }
            }

            State::ByY => match symbol {
                ' ' => state = State::NdRdSpaces,

                _ => {
                    state = State::Error;
                    return Err((index, "maybe you want to use space"));
                }
            }

            State::NdRdSpaces => {
                if symbol == ' ' {
                    state = State::NdRdSpaces;
                    index += 1;
                    continue;
                }

                if symbol == '0' {
                    signed_const_semantics.push(symbol);

                    state = State::RdConstZero;
                    index += 1;
                    continue;
                }

                if symbol == '-' {
                    signed_const_semantics.push(symbol);

                    state = State::RdConstMinus;
                    index += 1;
                    continue;
                }

                if DIGITS[1..].contains(&symbol) {
                    signed_const_semantics.push(symbol);

                    state = State::RdConst;
                    index += 1;
                    continue;
                }

                state = State::Error;
                return Err((index, "maybe you want to use \"0\", \"-\" or 1|..|9"));
            }

            State::RdConstMinus => {
                if DIGITS[1..].contains(&symbol) {
                    signed_const_semantics.push(symbol);

                    state = State::RdConst;
                    index += 1;
                    continue;
                }

                state = State::Error;
                return Err((index, "maybe you want to use 1|..|9"));
            }

            State::RdConst => {
                if symbol == ' ' {
                    if !signed_const_semantics.valid() {
                        state = State::Error;
                        return Err((index, "constant must be between -32768 and 32767"));
                    }

                    signed_const_semantics.save();

                    state = State::RdSpaces;
                    index += 1;
                    continue;
                }

                if DIGITS.contains(&symbol) {
                    signed_const_semantics.push(symbol);

                    state = State::RdConst;
                    index += 1;
                    continue;
                }

                state = State::Error;
                return Err((index, "maybe you want to use 0|..|9"));
            }

            State::RdConstZero => match symbol {
                ' ' => {
                    if !signed_const_semantics.valid() {
                        state = State::Error;
                        return Err((index, "constant must be between -32768 and 32767"));
                    }

                    signed_const_semantics.save();

                    state = State::RdSpaces
                },

                _ => {
                    state = State::Error;
                    return Err((index, "maybe you want to use space"));
                }
            }

            State::RdSpaces => match symbol {
                ' ' => state = State::RdSpaces,
                'd' => state = State::DoD,

                _ => {
                    state = State::Error;
                    return Err((index, "maybe you want to use \"d\""));
                }
            }

            State::DoD => match symbol {
                'o' => state = State::DoO,

                _ => {
                    state = State::Error;
                    return Err((index, "maybe you want to use \"o\""));
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
                return Err((index, "maybe you want to use terminal"));
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

    Ok(semantics_html(id_semantics, unsigned_const_semantics, signed_const_semantics))
}
