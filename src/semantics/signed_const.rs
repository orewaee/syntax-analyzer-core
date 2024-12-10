pub struct SignedConstSemantics {
    signed_const: String,
    pub vec: Vec<String>
}

impl SignedConstSemantics {
    pub fn new() -> Self {
        SignedConstSemantics {
            signed_const: String::new(),
            vec: Vec::new(),
        }
    }

    pub fn push(&mut self, symbol: char) {
        self.signed_const.push(symbol);
    }

    pub fn valid(&mut self) -> bool {
        let value = self.signed_const.parse::<i32>().unwrap();

        (value >= -32768) && (value <= 32767)
    }

    pub fn save(&mut self) {
        self.vec.push(self.signed_const.to_owned());
        self.signed_const = String::new();
    }
}
