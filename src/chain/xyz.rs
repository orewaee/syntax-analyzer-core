#[derive(Debug, PartialEq, Eq)]
enum State {
    S,
    A,
    B,
    C,
    F,
    E,
}

pub fn is_xyz(string: &str) -> Result<(i32, i32, i32), (usize, &str)> {
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
                    return Err((index, "maybe you wanted to start with x"));
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
                    return Err((index, "maybe you wanted to use y"));
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
                    return Err((index, "maybe you wanted to use z"));
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
                    return Err((index, "maybe you wanted to use ;"));
                }
            }

            State::E | State::F => {
                break;
            }
        }

        index += 1;
    }

    if state != State::F {
        return Err((index, "there should be a ; here"));
    }

    Ok((x, y, z))
}