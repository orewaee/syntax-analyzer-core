#[derive(Debug, Eq, PartialEq)]
pub enum State {
    Start,

    // "for" keyword
    ForF, ForO, ForR, ForSpaces,

    // identifier
    Id, IdSpaces,

    // square brackets
    LeftBracket, LeftBracketSpaces,
    RightBracket, RightBracketSpaces,

    // list items
    ListId, ListConst, ListSpaces,

    // ":=" construction
    Colon, Equal, ColonEqualSpaces,

    ConstSign, Const, ConstZero, ConstSpaces,

    ToT, ToO,

    Error,
    Finish
}