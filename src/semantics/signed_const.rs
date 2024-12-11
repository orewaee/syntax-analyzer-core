pub struct SignedConstSemantics {
    pub latest_index: usize,
    signed_const: String,
    pub vec: Vec<String>
}

impl SignedConstSemantics {
    pub fn new() -> Self {
        SignedConstSemantics {
            latest_index: 0,
            signed_const: String::new(),
            vec: Vec::new(),
        }
    }

    pub fn push(&mut self, symbol: char) {
        self.signed_const.push(symbol);
    }

    pub fn update_index(&mut self, index: usize) {
        self.latest_index = index;
    }

    pub fn valid(&mut self) -> bool {
        let value = self.signed_const.parse::<i32>().unwrap();

        (value >= -32768) && (value <= 32767)
    }

    pub fn save(&mut self) {
        self.vec.push(self.signed_const.to_owned());
        self.signed_const = String::new();
    }

    pub fn get_range(&mut self) -> (i32, i32, i32) {
        let len = self.vec.len();

        let from = self.vec[0].parse::<i32>().unwrap();
        let to = self.vec[1].parse::<i32>().unwrap();

        let mut step = 1;
        if len == 3 {
            step = self.vec[2].parse::<i32>().unwrap();
        }

        (from, to, step)
    }

    pub fn check_range(&mut self) -> bool {
        let (from, to, step) = self.get_range();

        if step > 0 && from > to {
            return false;
        }

        if step < 0 && from < to {
            return false;
        }

        true
    }

    pub fn iterations(&mut self) -> i32 {
        let (from, to, step) = self.get_range();

        if from == to {
            return 1;
        }

        i32::abs((from - to) / step)
    }
}
