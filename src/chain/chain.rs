#[derive(Debug, Eq, PartialEq)]
enum State {
    Start,
    F, O, R,
    Spaces0,
    IdLetter0,
    IdLetterOrDigit0,
    Colon,
    Equal,
    Spaces1,
    ConstWithSign,

    // negative constant states
    NegativeConst,
    NC0, NC1, NC2, NC3, NC4, NC5, NC6, NC7, NC8,

    // zero constant
    ZeroConst,

    // positive constant states
    PositiveConst,
    PC0, PC1, PC2, PC3, PC4, PC5, PC6, PC7, PC8,

    Error,
    Finish
}

fn is_for(string: &str) -> Result<(), (usize, &str)> {
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
                    continue
                }

                'f' => {
                    state = State::F;
                    continue;
                }

                _ => {
                    state = State::Error;
                    return Err((index, "error"));
                }
            }

            State::F => match symbol {
                'o' => {
                    state = State::O;
                    continue;
                }

                _ => {
                    state = State::Error;
                    return Err((index, "error"));
                }
            }

            State::O => match symbol {
                'r' => {
                    state = State::R;
                    continue;
                }

                _ => {
                    state = State::Error;
                    return Err((index, "error"));
                }
            }

            State::R => match symbol {
                ' ' => {
                    state = State::Spaces0;
                    continue;
                }

                _ => {
                    state = State::Error;
                    return Err((index, "error"));
                }
            }

            State::Spaces0 => {
                if symbol == ' ' {
                    state = State::Spaces0;
                    continue;
                }

                if letters.contains(&symbol) {
                    state = State::IdLetter0;
                    continue;
                }

                state = State::Error;
                return Err((index, "error"));
            }

            State::IdLetter0 => {
                if letters.contains(&symbol) || digits.contains(&symbol) {
                    state = State::IdLetterOrDigit0;
                    continue;
                }

                state = State::Error;
                return Err((index, "error"));
            }

            State::IdLetterOrDigit0 => {
                if letters.contains(&symbol) || digits.contains(&symbol) {
                    state = State::IdLetterOrDigit0;
                    continue;
                }

                if symbol == ':' {
                    state = State::Colon;
                    continue;
                }

                state = State::Error;
                return Err((index, "error"));
            }

            State::Colon => {
                if symbol == '=' {
                    state = State::Equal;
                    continue;
                }

                state = State::Error;
                return Err((index, "error"));
            }

            State::Equal => {
                if symbol == ' ' {
                    state = State::Spaces1;
                    continue;
                }



                state = State::Error;
                return Err((index, "error"));
            }

            State::Spaces1 => {
                if symbol == ' ' {
                    state = State::Spaces1;
                    continue;
                }

                if symbol == '0' {
                    state = State::ZeroConst;
                    continue
                }

                if symbol == '-' {
                    state = State::NegativeConst;
                    continue;
                }

                if symbol == '+' {
                    state = State::PositiveConst;
                    continue;
                }

                state = State::Error;
                return Err((index, "error"));
            }

            // negative constant

            State::NegativeConst => match symbol {
                '1' | '2' => state = State::NC0,
                '3' => state = State::NC1,

                _ => {
                    state = State::Error;
                    return Err((index, "error"));
                }
            }

            State::NC0 => match symbol {
                '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => state = State::NC2,
                '*' => {}, // todo replace to end

                _ => {
                    state = State::Error;
                    return Err((index, "error"));
                }
            }

            State::NC1 => match symbol {
                '0' | '1' => state = State::NC2,
                '2' => state = State::NC3,
                '*' => {}, // todo replace to end

                _ => {
                    state = State::Error;
                    return Err((index, "error"));
                }
            }

            State::NC2 => match symbol {
                '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => state = State::NC4,
                '*' => {}, // todo replace to end

                _ => {
                    state = State::Error;
                    return Err((index, "error"));
                }
            }

            State::NC3 => match symbol {
                '0' | '1' | '2' | '3' | '4' | '5' | '6' => state = State::NC4,
                '7' => state = State::NC5,
                '*' => {}, // todo replace to end

                _ => {
                    state = State::Error;
                    return Err((index, "error"));
                }
            }

            State::NC4 => match symbol {
                '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => state = State::NC6,
                '*' => {}, // todo replace to end

                _ => {
                    state = State::Error;
                    return Err((index, "error"));
                }
            }

            State::NC5 => match symbol {
                '0' | '1' | '2' | '3' | '4' | '5' => state = State::NC6,
                '6' => state = State::NC7,
                '*' => {}, // todo replace to end

                _ => {
                    state = State::Error;
                    return Err((index, "error"));
                }
            }

            State::NC6 => match symbol {
                '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => state = State::NC8,
                '*' => {}, // todo replace to end

                _ => {
                    state = State::Error;
                    return Err((index, "error"));
                }
            }

            State::NC7 => match symbol {
                '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' => state = State::NC8,
                '*' => {}, // todo replace to end

                _ => {
                    state = State::Error;
                    return Err((index, "error"));
                }
            }

            State::NC8 => match symbol {
                '*' => {}, // todo replace to end

                _ => {
                    state = State::Error;
                    return Err((index, "error"));
                }
            }

            // zero constant

            State::ZeroConst => match symbol {
                '*' => {}, // todo replace to end

                _ => {
                    state = State::Error;
                    return Err((index, "error"));
                }
            }

            // positive constant

            State::PositiveConst => match symbol {
                '1' | '2' => state = State::PC0,
                '3' => state = State::PC1,

                _ => {
                    state = State::Error;
                    return Err((index, "error"));
                }
            }

            State::PC0 => match symbol {
                '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => state = State::PC2,
                '*' => {}, // todo replace to end

                _ => {
                    state = State::Error;
                    return Err((index, "error"));
                }
            }

            State::PC1 => match symbol {
                '0' | '1' => state = State::PC2,
                '2' => state = State::PC3,
                '*' => {}, // todo replace to end

                _ => {
                    state = State::Error;
                    return Err((index, "error"));
                }
            }

            State::PC2 => match symbol {
                '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => state = State::PC4,
                '*' => {}, // todo replace to end

                _ => {
                    state = State::Error;
                    return Err((index, "error"));
                }
            }

            State::PC3 => match symbol {
                '0' | '1' | '2' | '3' | '4' | '5' | '6' => state = State::PC4,
                '7' => state = State::PC5,
                '*' => {}, // todo replace to end

                _ => {
                    state = State::Error;
                    return Err((index, "error"));
                }
            }

            State::PC4 => match symbol {
                '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => state = State::PC6,
                '*' => {}, // todo replace to end

                _ => {
                    state = State::Error;
                    return Err((index, "error"));
                }
            }

            State::PC5 => match symbol {
                '0' | '1' | '2' | '3' | '4' | '5' => state = State::PC6,
                '6' => state = State::PC7,
                '*' => {}, // todo replace to end

                _ => {
                    state = State::Error;
                    return Err((index, "error"));
                }
            }

            State::PC6 => match symbol {
                '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => state = State::PC8,
                '*' => {}, // todo replace to end

                _ => {
                    state = State::Error;
                    return Err((index, "error"));
                }
            }

            State::PC7 => match symbol {
                '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' => state = State::PC8,
                '*' => {}, // todo replace to end

                _ => {
                    state = State::Error;
                    return Err((index, "error"));
                }
            }

            State::PC8 => match symbol {
                '*' => {}, // todo replace to end

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
    }

    Ok(())
}
