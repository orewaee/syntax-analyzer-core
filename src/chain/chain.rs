use crate::chain::state::State;

pub fn is_for(string: &str, end_terminal: char) -> Result<(), (usize, &str)> {
    let mut state = State::Start;
    let mut index = 0;
    let mut symbol: char;

    let digits: Vec<char> = "0123456789".chars().collect();
    let letters: Vec<char> = "abcdefghijklmnopqrstuvwxyz".chars().collect();

    while state != State::Finish && state != State::Error && index < string.len() {
        symbol = string.chars().nth(index).unwrap().to_ascii_lowercase();

        match state {
            State::Start => match symbol {
                ' ' => {
                    state = State::Start;
                    index += 1;
                    continue
                }

                'f' => {
                    state = State::ForF;
                    index += 1;
                    continue;
                }

                _ => {
                    state = State::Error;
                    return Err((index, "maybe you want to use F"));
                }
            }

            State::ForF => match symbol {
                'o' => {
                    state = State::ForO;
                    index += 1;
                    continue;
                }

                _ => {
                    state = State::Error;
                    return Err((index, "maybe you want to use O, but expected "));
                }
            }

            State::ForO => match symbol {
                'r' => {
                    state = State::ForR;
                    index += 1;
                    continue;
                }

                _ => {
                    state = State::Error;
                    return Err((index, "error"));
                }
            }

            State::ForR => match symbol {
                ' ' => {
                    state = State::ForSpaces;
                    index += 1;
                    continue;
                }

                _ => {
                    state = State::Error;
                    return Err((index, "error"));
                }
            }

            State::ForSpaces => {
                if symbol == ' ' {
                    state = State::ForSpaces;
                    index += 1;
                    continue;
                }

                if letters.contains(&symbol) {
                    state = State::IdLetter;
                    index += 1;
                    continue;
                }

                state = State::Error;
                return Err((index, "error"));
            }

            State::IdLetter => {
                if letters.contains(&symbol) || digits.contains(&symbol) {
                    state = State::IdLetterOrDigit;
                    index += 1;
                    continue;
                }

                state = State::Error;
                return Err((index, "error"));
            }

            State::IdLetterOrDigit => {
                if letters.contains(&symbol) || digits.contains(&symbol) {
                    state = State::IdLetterOrDigit;
                    index += 1;
                    continue;
                }

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
                    state = State::LeftBracketSpaces;
                    index += 1;
                    continue;
                }

                if letters.contains(&symbol) {
                    state = State::ListIdLetter;
                    index += 1;
                    continue;
                }

                if symbol == '1' {
                    state = State::LC0;
                    index += 1;
                    continue;
                }

                if symbol == '2' {
                    state = State::LC1;
                    index += 1;
                    continue;
                }

                if digits[3..].contains(&symbol) {
                    state = State::LC2;
                    index += 1;
                    continue;
                }

                state = State::Error;
                return Err((index, "error"));
            }

            State::LeftBracketSpaces => {
                if symbol == ' ' {
                    state = State::LeftBracketSpaces;
                    index += 1;
                    continue;
                }

                if letters.contains(&symbol) {
                    state = State::ListIdLetter;
                    index += 1;
                    continue;
                }

                if symbol == '1' {
                    state = State::LC0;
                    index += 1;
                    continue;
                }

                if symbol == '2' {
                    state = State::LC1;
                    index += 1;
                    continue;
                }

                if digits[3..].contains(&symbol) {
                    state = State::LC2;
                    index += 1;
                    continue;
                }

                state = State::Error;
                return Err((index, "error"));
            }

            /*
            State::LC0 => match symbol {
                '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {

                }
            }
            */

            State::ListIdLetter => {
                if letters.contains(&symbol) || digits.contains(&symbol) {
                    state = State::ListIdLetterOrDigit;
                    index += 1;
                    continue;
                }

                state = State::Error;
                return Err((index, "error"));
            }

            State::ListIdLetterOrDigit => match symbol {
                ' ' => state = State::RightBracketSpaces,
                ',' => state = State::Comma,
                ']' => state = State::RightBracket,

                _ => {
                    state = State::Error;
                    return Err((index, "error"));
                }
            }

            State::Comma | State::CommaSpaces => {
                if symbol == ' ' {
                    state = State::CommaSpaces;
                    index += 1;
                    continue;
                }

                if letters.contains(&symbol) {
                    state = State::ListIdLetter;
                    index += 1;
                    continue;
                }

                if symbol == '1' {
                    state = State::LC0;
                    index += 1;
                    continue;
                }

                if symbol == '2' {
                    state = State::LC1;
                    index += 1;
                    continue;
                }

                if digits[3..].contains(&symbol) {
                    state = State::LC2;
                    index += 1;
                    continue;
                }

                state = State::Error;
                return Err((index, "error"));
            }

            State::Colon => match symbol {
                '=' => state = State::Equal,

                _ => {
                    state = State::Error;
                    return Err((index, "error"));
                }
            }

            State::Equal => match symbol {
                ' ' => state = State::ColonEqualSpaces,

                _ => {
                    state = State::Error;
                    return Err((index, "error"));
                }
            }

            State::ColonEqualSpaces => match symbol {
                ' ' => state = State::ColonEqualSpaces,
                '-' => state = State::StNegativeConst,
                '0' => state = State::StZeroConst,
                '+' => state = State::StPositiveConst,

                _ => {
                    state = State::Error;
                    return Err((index, "error"));
                }
            }

            // negative constant

            State::StNegativeConst => match symbol {
                '1' | '2' => state = State::StNC0,
                '3' => state = State::StNC1,

                _ => {
                    state = State::Error;
                    return Err((index, "error"));
                }
            }

            State::StNC0 => match symbol {
                '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => state = State::StNC2,

                ' ' => state = State::StSpaces,
                't' => state = State::ToT,

                _ => {
                    state = State::Error;
                    return Err((index, "error"));
                }
            }

            State::StNC1 => match symbol {
                '0' | '1' => state = State::StNC2,
                '2' => state = State::StNC3,

                ' ' => state = State::StSpaces,
                't' => state = State::ToT,

                _ => {
                    state = State::Error;
                    return Err((index, "error"));
                }
            }

            State::StNC2 => match symbol {
                '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => state = State::StNC4,

                ' ' => state = State::StSpaces,
                't' => state = State::ToT,

                _ => {
                    state = State::Error;
                    return Err((index, "error"));
                }
            }

            State::StNC3 => match symbol {
                '0' | '1' | '2' | '3' | '4' | '5' | '6' => state = State::StNC4,
                '7' => state = State::StNC5,

                ' ' => state = State::StSpaces,
                't' => state = State::ToT,

                _ => {
                    state = State::Error;
                    return Err((index, "error"));
                }
            }

            State::StNC4 => match symbol {
                '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => state = State::StNC6,

                ' ' => state = State::StSpaces,
                't' => state = State::ToT,

                _ => {
                    state = State::Error;
                    return Err((index, "error"));
                }
            }

            State::StNC5 => match symbol {
                '0' | '1' | '2' | '3' | '4' | '5' => state = State::StNC6,
                '6' => state = State::StNC7,

                ' ' => state = State::StSpaces,
                't' => state = State::ToT,

                _ => {
                    state = State::Error;
                    return Err((index, "error"));
                }
            }

            State::StNC6 => match symbol {
                '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => state = State::StNC8,

                ' ' => state = State::StSpaces,
                't' => state = State::ToT,

                _ => {
                    state = State::Error;
                    return Err((index, "error"));
                }
            }

            State::StNC7 => match symbol {
                '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' => state = State::StNC8,

                ' ' => state = State::StSpaces,
                't' => state = State::ToT,

                _ => {
                    state = State::Error;
                    return Err((index, "error"));
                }
            }

            State::StNC8 | State::StSpaces => match symbol {
                ' ' => state = State::StSpaces,
                't' => state = State::ToT,

                _ => {
                    state = State::Error;
                    return Err((index, "error"));
                }
            }

            State::NdNegativeConst => match symbol {
                '1' | '2' => state = State::NdNC0,
                '3' => state = State::NdNC1,

                _ => {
                    state = State::Error;
                    return Err((index, "error"));
                }
            }

            State::NdNC0 => match symbol {
                '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => state = State::NdNC2,

                ' ' => state = State::NdSpaces,
                'b' => state = State::ByB,
                'd' => state = State::DoD,

                _ => {
                    state = State::Error;
                    return Err((index, "error"));
                }
            }

            State::NdNC1 => match symbol {
                '0' | '1' => state = State::NdNC2,
                '2' => state = State::NdNC3,

                ' ' => state = State::NdSpaces,
                'b' => state = State::ByB,
                'd' => state = State::DoD,

                _ => {
                    state = State::Error;
                    return Err((index, "error"));
                }
            }

            State::NdNC2 => match symbol {
                '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => state = State::NdNC4,

                ' ' => state = State::NdSpaces,
                'b' => state = State::ByB,
                'd' => state = State::DoD,

                _ => {
                    state = State::Error;
                    return Err((index, "error"));
                }
            }

            State::NdNC3 => match symbol {
                '0' | '1' | '2' | '3' | '4' | '5' | '6' => state = State::NdNC4,
                '7' => state = State::NdNC5,

                ' ' => state = State::NdSpaces,
                'b' => state = State::ByB,
                'd' => state = State::DoD,

                _ => {
                    state = State::Error;
                    return Err((index, "error"));
                }
            }

            State::NdNC4 => match symbol {
                '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => state = State::NdNC6,

                ' ' => state = State::NdSpaces,
                'b' => state = State::ByB,
                'd' => state = State::DoD,

                _ => {
                    state = State::Error;
                    return Err((index, "error"));
                }
            }

            State::NdNC5 => match symbol {
                '0' | '1' | '2' | '3' | '4' | '5' => state = State::NdNC6,
                '6' => state = State::NdNC7,

                ' ' => state = State::NdSpaces,
                'b' => state = State::ByB,
                'd' => state = State::DoD,

                _ => {
                    state = State::Error;
                    return Err((index, "error"));
                }
            }

            State::NdNC6 => match symbol {
                '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => state = State::NdNC8,

                ' ' => state = State::NdSpaces,
                'b' => state = State::ByB,
                'd' => state = State::DoD,

                _ => {
                    state = State::Error;
                    return Err((index, "error"));
                }
            }

            State::NdNC7 => match symbol {
                '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' => state = State::NdNC8,

                ' ' => state = State::NdSpaces,
                'b' => state = State::ByB,
                'd' => state = State::DoD,

                _ => {
                    state = State::Error;
                    return Err((index, "error"));
                }
            }

            State::NdNC8 | State::NdSpaces => match symbol {
                ' ' => state = State::NdSpaces,
                'b' => state = State::ByB,
                'd' => state = State::DoD,

                _ => {
                    state = State::Error;
                    return Err((index, "error"));
                }
            }

            State::RdNegativeConst => match symbol {
                '1' | '2' => state = State::RdNC0,
                '3' => state = State::RdNC1,

                _ => {
                    state = State::Error;
                    return Err((index, "error"));
                }
            }

            State::RdNC0 => match symbol {
                '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => state = State::RdNC2,

                ' ' => state = State::RdSpaces,
                'd' => state = State::DoD,

                _ => {
                    state = State::Error;
                    return Err((index, "error"));
                }
            }

            State::RdNC1 => match symbol {
                '0' | '1' => state = State::RdNC2,
                '2' => state = State::RdNC3,

                ' ' => state = State::RdSpaces,
                'd' => state = State::DoD,

                _ => {
                    state = State::Error;
                    return Err((index, "error"));
                }
            }

            State::RdNC2 => match symbol {
                '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => state = State::RdNC4,

                ' ' => state = State::RdSpaces,
                'd' => state = State::DoD,

                _ => {
                    state = State::Error;
                    return Err((index, "error"));
                }
            }

            State::RdNC3 => match symbol {
                '0' | '1' | '2' | '3' | '4' | '5' | '6' => state = State::RdNC4,
                '7' => state = State::RdNC5,

                ' ' => state = State::RdSpaces,
                'd' => state = State::DoD,

                _ => {
                    state = State::Error;
                    return Err((index, "error"));
                }
            }

            State::RdNC4 => match symbol {
                '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => state = State::RdNC6,

                ' ' => state = State::RdSpaces,
                'd' => state = State::DoD,

                _ => {
                    state = State::Error;
                    return Err((index, "error"));
                }
            }

            State::RdNC5 => match symbol {
                '0' | '1' | '2' | '3' | '4' | '5' => state = State::RdNC6,
                '6' => state = State::RdNC7,

                ' ' => state = State::RdSpaces,
                'd' => state = State::DoD,

                _ => {
                    state = State::Error;
                    return Err((index, "error"));
                }
            }

            State::RdNC6 => match symbol {
                '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => state = State::RdNC8,

                ' ' => state = State::RdSpaces,
                'd' => state = State::DoD,

                _ => {
                    state = State::Error;
                    return Err((index, "error"));
                }
            }

            State::RdNC7 => match symbol {
                '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' => state = State::RdNC8,

                ' ' => state = State::RdSpaces,
                'd' => state = State::DoD,

                _ => {
                    state = State::Error;
                    return Err((index, "error"));
                }
            }

            State::RdNC8 | State::RdSpaces => match symbol {
                ' ' => state = State::RdSpaces,
                'd' => state = State::DoD,

                _ => {
                    state = State::Error;
                    return Err((index, "error"));
                }
            }

            // zero constant

            State::StZeroConst => match symbol {
                ' ' => state = State::StSpaces,
                't' => state = State::ToT,

                _ => {
                    state = State::Error;
                    return Err((index, "error"));
                }
            }

            State::NdZeroConst => match symbol {
                ' ' => state = State::NdSpaces,
                'b' => state = State::ByB,
                'd' => state = State::DoD,

                _ => {
                    state = State::Error;
                    return Err((index, "error"));
                }
            }

            State::RdZeroConst => match symbol {
                ' ' => state = State::RdSpaces,
                'd' => state = State::DoD,

                _ => {
                    state = State::Error;
                    return Err((index, "error"));
                }
            }

            // positive constant

            State::StPositiveConst => match symbol {
                '1' | '2' => state = State::StPC0,
                '3' => state = State::StPC1,

                _ => {
                    state = State::Error;
                    return Err((index, "error"));
                }
            }

            State::StPC0 => match symbol {
                '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => state = State::StPC2,

                ' ' => state = State::StSpaces,
                't' => state = State::ToT,

                _ => {
                    state = State::Error;
                    return Err((index, "error"));
                }
            }

            State::StPC1 => match symbol {
                '0' | '1' => state = State::StPC2,
                '2' => state = State::StPC3,

                ' ' => state = State::StSpaces,
                't' => state = State::ToT,

                _ => {
                    state = State::Error;
                    return Err((index, "error"));
                }
            }

            State::StPC2 => match symbol {
                '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => state = State::StPC4,

                ' ' => state = State::StSpaces,
                't' => state = State::ToT,

                _ => {
                    state = State::Error;
                    return Err((index, "error"));
                }
            }

            State::StPC3 => match symbol {
                '0' | '1' | '2' | '3' | '4' | '5' | '6' => state = State::StPC4,
                '7' => state = State::StPC5,

                ' ' => state = State::StSpaces,
                't' => state = State::ToT,

                _ => {
                    state = State::Error;
                    return Err((index, "error"));
                }
            }

            State::StPC4 => match symbol {
                '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => state = State::StPC6,

                ' ' => state = State::StSpaces,
                't' => state = State::ToT,

                _ => {
                    state = State::Error;
                    return Err((index, "error"));
                }
            }

            State::StPC5 => match symbol {
                '0' | '1' | '2' | '3' | '4' | '5' => state = State::StPC6,
                '6' => state = State::StPC7,

                ' ' => state = State::StSpaces,
                't' => state = State::ToT,

                _ => {
                    state = State::Error;
                    return Err((index, "error"));
                }
            }

            State::StPC6 => match symbol {
                '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => state = State::StPC8,

                ' ' => state = State::StSpaces,
                't' => state = State::ToT,

                _ => {
                    state = State::Error;
                    return Err((index, "error"));
                }
            }

            State::StPC7 => match symbol {
                '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' => state = State::StPC8,

                ' ' => state = State::StSpaces,
                't' => state = State::ToT,

                _ => {
                    state = State::Error;
                    return Err((index, "error"));
                }
            }

            State::StPC8 => match symbol {
                ' ' => state = State::StSpaces,
                't' => state = State::ToT,

                _ => {
                    state = State::Error;
                    return Err((index, "error"));
                }
            }

            State::NdPositiveConst => match symbol {
                '1' | '2' => state = State::NdPC0,
                '3' => state = State::NdPC1,

                _ => {
                    state = State::Error;
                    return Err((index, "error"));
                }
            }

            State::NdPC0 => match symbol {
                '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => state = State::NdPC2,

                ' ' => state = State::NdSpaces,
                'b' => state = State::ByB,
                'd' => state = State::DoD,

                _ => {
                    state = State::Error;
                    return Err((index, "error"));
                }
            }

            State::NdPC1 => match symbol {
                '0' | '1' => state = State::NdPC2,
                '2' => state = State::NdPC3,

                ' ' => state = State::NdSpaces,
                'b' => state = State::ByB,
                'd' => state = State::DoD,

                _ => {
                    state = State::Error;
                    return Err((index, "error"));
                }
            }

            State::NdPC2 => match symbol {
                '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => state = State::NdPC4,

                ' ' => state = State::NdSpaces,
                'b' => state = State::ByB,
                'd' => state = State::DoD,

                _ => {
                    state = State::Error;
                    return Err((index, "error"));
                }
            }

            State::NdPC3 => match symbol {
                '0' | '1' | '2' | '3' | '4' | '5' | '6' => state = State::NdPC4,
                '7' => state = State::NdPC5,

                ' ' => state = State::NdSpaces,
                'b' => state = State::ByB,
                'd' => state = State::DoD,

                _ => {
                    state = State::Error;
                    return Err((index, "error"));
                }
            }

            State::NdPC4 => match symbol {
                '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => state = State::NdPC6,

                ' ' => state = State::NdSpaces,
                'b' => state = State::ByB,
                'd' => state = State::DoD,

                _ => {
                    state = State::Error;
                    return Err((index, "error"));
                }
            }

            State::NdPC5 => match symbol {
                '0' | '1' | '2' | '3' | '4' | '5' => state = State::NdPC6,
                '6' => state = State::NdPC7,

                ' ' => state = State::NdSpaces,
                'b' => state = State::ByB,
                'd' => state = State::DoD,

                _ => {
                    state = State::Error;
                    return Err((index, "error"));
                }
            }

            State::NdPC6 => match symbol {
                '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => state = State::NdPC8,

                ' ' => state = State::NdSpaces,
                'b' => state = State::ByB,
                'd' => state = State::DoD,

                _ => {
                    state = State::Error;
                    return Err((index, "error"));
                }
            }

            State::NdPC7 => match symbol {
                '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' => state = State::NdPC8,

                ' ' => state = State::NdSpaces,
                'b' => state = State::ByB,
                'd' => state = State::DoD,

                _ => {
                    state = State::Error;
                    return Err((index, "error"));
                }
            }

            State::NdPC8 => match symbol {
                ' ' => state = State::NdSpaces,
                'b' => state = State::ByB,
                'd' => state = State::DoD,

                _ => {
                    state = State::Error;
                    return Err((index, "error"));
                }
            }

            State::RdPositiveConst => match symbol {
                '1' | '2' => state = State::RdPC0,
                '3' => state = State::RdPC1,

                _ => {
                    state = State::Error;
                    return Err((index, "error"));
                }
            }

            State::RdPC0 => match symbol {
                '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => state = State::RdPC2,

                ' ' => state = State::RdSpaces,
                'd' => state = State::DoD,

                _ => {
                    state = State::Error;
                    return Err((index, "error"));
                }
            }

            State::RdPC1 => match symbol {
                '0' | '1' => state = State::RdPC2,
                '2' => state = State::RdPC3,

                ' ' => state = State::RdSpaces,
                'd' => state = State::DoD,

                _ => {
                    state = State::Error;
                    return Err((index, "error"));
                }
            }

            State::RdPC2 => match symbol {
                '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => state = State::RdPC4,

                ' ' => state = State::RdSpaces,
                'd' => state = State::DoD,

                _ => {
                    state = State::Error;
                    return Err((index, "error"));
                }
            }

            State::RdPC3 => match symbol {
                '0' | '1' | '2' | '3' | '4' | '5' | '6' => state = State::RdPC4,
                '7' => state = State::RdPC5,

                ' ' => state = State::RdSpaces,
                'd' => state = State::DoD,

                _ => {
                    state = State::Error;
                    return Err((index, "error"));
                }
            }

            State::RdPC4 => match symbol {
                '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => state = State::RdPC6,

                ' ' => state = State::RdSpaces,
                'd' => state = State::DoD,

                _ => {
                    state = State::Error;
                    return Err((index, "error"));
                }
            }

            State::RdPC5 => match symbol {
                '0' | '1' | '2' | '3' | '4' | '5' => state = State::RdPC6,
                '6' => state = State::RdPC7,

                ' ' => state = State::RdSpaces,
                'd' => state = State::DoD,

                _ => {
                    state = State::Error;
                    return Err((index, "error"));
                }
            }

            State::RdPC6 => match symbol {
                '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => state = State::RdPC8,

                ' ' => state = State::RdSpaces,
                'd' => state = State::DoD,

                _ => {
                    state = State::Error;
                    return Err((index, "error"));
                }
            }

            State::RdPC7 => match symbol {
                '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' => state = State::RdPC8,

                ' ' => state = State::RdSpaces,
                'd' => state = State::DoD,

                _ => {
                    state = State::Error;
                    return Err((index, "error"));
                }
            }

            State::RdPC8 => match symbol {
                ' ' => state = State::RdSpaces,
                'd' => state = State::DoD,

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

            State::ToO | State::StNdSpaces => match symbol {
                ' ' => state = State::StNdSpaces,
                '-' => state = State::NdNegativeConst,
                '0' => state = State::NdZeroConst,
                '+' => state = State::NdPositiveConst,

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

            State::ByY | State::NdRdSpaces => match symbol {
                ' ' => state = State::NdRdSpaces,
                '-' => state = State::RdNegativeConst,
                '0' => state = State::RdZeroConst,
                '+' => state = State::RdPositiveConst,

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
                if symbol == end_terminal {
                    state = State::Finish;
                    index += 1;
                    continue;
                }

                state = State::Error;
                return Err((index, "error"));
            }

            _ => {
                state = State::Error;
                return Err((index, "unexpected error"));
            }
        }

        index += 1;
    }

    if state != State::Finish {
        return Err((index, "use end terminal for close chain"));
    }

    Ok(())
}
