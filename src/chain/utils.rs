use crate::chain::state::State;

// 32767
// 01234

pub fn negative_const_0(
    symbol: char,
    one_or_two: State,
    three: State
) -> Result<State, ()> {
    match symbol {
        '1' | '2' => Ok(one_or_two),
        '3' => Ok(three),

        _ => Err(())
    }
}
