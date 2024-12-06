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

    // constants
    StConstMinus, StConst, StConstZero,
    NdConstMinus, NdConst, NdConstZero,
    RdConstMinus, RdConst, RdConstZero,

    // indents before and after constants
    StSpaces, StNdSpaces, NdSpaces, NdRdSpaces, RdSpaces,

    // "to", "by" and "do" constants
    ToT, ToO,
    ByB, ByY,
    DoD, DoO,

    Error,
    Finish
}