pub struct IdSemantics {
    id: String,
    pub ids: Vec<String>
}

impl IdSemantics {
    pub fn new() -> IdSemantics {
        IdSemantics {
            id: String::new(),
            ids: Vec::new()
        }
    }

    pub fn push_symbol(&mut self, symbol: char) {
        self.id.push(symbol);
    }

    pub fn  valid_length(&mut self) -> bool {
        (self.id.len() > 0) && (self.id.len() <= 8)
    }

    pub fn has_keyword(&mut self) -> bool {
        let keywords = vec!["for", "to", "by", "do"];

        for keyword in keywords {
            if self.id.contains(keyword) {
                return true;
            }
        }

        false
    }

    pub fn already_exists(&mut self) -> bool {
        self.ids.contains(&self.id)
    }

    pub fn validate(&mut self) -> Result<(), String> {
        if !self.valid_length() {
            return Err(String::from("id length should be from 1 to 8 chars"));
        }

        if !self.has_keyword() {
            return Err(String::from("id should not include keywords"));
        }

        if !self.already_exists() {
            return Err(String::from("ids must not be repeated"));
        }

        Ok(())
    }

    pub fn save_id(&mut self) {
        self.ids.push(self.id.to_owned());
        self.id = String::new();
    }
}
