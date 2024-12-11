pub struct IdSemantics {
    id: String,
    pub vec: Vec<String>
}

impl IdSemantics {
    pub fn new() -> IdSemantics {
        IdSemantics {
            id: String::new(),
            vec: Vec::new()
        }
    }

    pub fn push(&mut self, symbol: char) {
        self.id.push(symbol);
    }

    pub fn valid_length(&mut self) -> bool {
        (self.id.len() > 0) && (self.id.len() <= 8)
    }

    pub fn eq_keyword(&mut self) -> bool {
        let keywords = vec!["for", "to", "by", "do"];

        for keyword in keywords {
            if self.id.eq_ignore_ascii_case(&keyword) {
                return true;
            }
        }

        false
    }

    pub fn already_exists(&mut self) -> bool {
        self.vec.contains(&self.id)
    }

    pub fn save(&mut self) {
        self.vec.push(self.id.to_owned());
        self.id = String::new();
    }

    pub fn semantics(&mut self) -> (String, Vec<String>) {
        let ids = self.vec[1..].to_vec();

        (self.vec[0].to_owned(), ids)
    }
}
