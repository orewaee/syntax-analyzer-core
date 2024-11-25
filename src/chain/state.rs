#[derive(Debug, Eq, PartialEq)]
pub enum State {
    Start,

    // "for" keyword
    ForF, ForO, ForR, ForSpaces,


    // identifier
    IdLetter, IdLetterOrDigit, IdSpaces,


    // list identifier
    ListIdLetter, ListIdLetterOrDigit,


    // list constant
    LC0, LC1, LC2, LC3, LC4, LC5, LC6,


    // ":=" construction
    Colon, Equal, ColonEqualSpaces,


    // list comma
    Comma, CommaSpaces,

    // square brackets
    LeftBracket, LeftBracketSpaces,
    RightBracket, RightBracketSpaces,


    // negative constant
    StNegativeConst,
    StNC0, StNC1, StNC2, StNC3, StNC4, StNC5, StNC6, StNC7, StNC8,

    NdNegativeConst,
    NdNC0, NdNC1, NdNC2, NdNC3, NdNC4, NdNC5, NdNC6, NdNC7, NdNC8,

    RdNegativeConst,
    RdNC0, RdNC1, RdNC2, RdNC3, RdNC4, RdNC5, RdNC6, RdNC7, RdNC8,


    // zero constant
    StZeroConst, NdZeroConst, RdZeroConst,


    // positive constant
    StPositiveConst,
    StPC0, StPC1, StPC2, StPC3, StPC4, StPC5, StPC6, StPC7, StPC8,

    NdPositiveConst,
    NdPC0, NdPC1, NdPC2, NdPC3, NdPC4, NdPC5, NdPC6, NdPC7, NdPC8,

    RdPositiveConst,
    RdPC0, RdPC1, RdPC2, RdPC3, RdPC4, RdPC5, RdPC6, RdPC7, RdPC8,


    // constant spaces
    StSpaces, StNdSpaces, NdSpaces, NdRdSpaces, RdSpaces,


    // "to" keyword
    ToT, ToO,


    // "by" keyword
    ByB, ByY,


    // "do" keyword
    DoD, DoO,


    Error,
    Finish
}
