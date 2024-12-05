#[derive(Debug, Eq, PartialEq)]
pub enum State {
    Start,

    // "for" keyword
    ForF,
    ForO,
    ForR,
    ForSpaces,

    // identifier
    IdLetter,
    IdLetterOrDigit,
    IdSpaces,

    // square brackets
    LeftBracket,
    LeftBracketSpaces,
    RightBracket,
    RightBracketSpaces,

    // list identifier
    ListIdLetter,
    ListIdLetterOrDigit,

    // list constant
    ListConstHead,
    ListConstBody,

    ListSpaces,

    // ":=" construction
    Colon, Equal, ColonEqualSpaces,

    // list comma
    Comma, CommaSpaces,

    Error,
    Finish
}