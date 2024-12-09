pub struct UnsignedConstSemantics {
    unsigned_const: String,
    pub vec: Vec<String>
}

impl UnsignedConstSemantics {
    pub fn new() -> UnsignedConstSemantics {
        UnsignedConstSemantics {
            unsigned_const: String::new(),
            vec: Vec::new()
        }
    }

    pub fn push(&mut self, symbol: char) {
        self.unsigned_const.push(symbol);
    }

    pub fn valid(&mut self) -> bool {
        let value = self.unsigned_const.parse::<i32>().unwrap();

        (value > 0) && (value <= 256)
    }

    pub fn save(&mut self) {
        self.vec.push(self.unsigned_const.to_owned());
        self.unsigned_const = String::new();
    }
}
