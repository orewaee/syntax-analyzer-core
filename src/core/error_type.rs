#[derive(Debug, Eq, PartialEq)]
pub enum ErrorType {
    Syntax,
    Semantics
}

impl ErrorType {
    pub fn plain(self) -> String {
        match self {
            ErrorType::Syntax => "syntax error".to_string(),
            ErrorType::Semantics => "semantics error".to_string()
        }
    }
}
