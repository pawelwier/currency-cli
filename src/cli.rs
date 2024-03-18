pub struct Cli {
    pub source_currency: String,
    pub target_currency: String,
    pub amount: f32
}

impl Cli {
    pub fn print_res(&self) -> () {
        println!("src: {}, trg: {}, amount: {}", self.source_currency, self.target_currency, self.amount);
    }
}