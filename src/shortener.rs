use harsh::Harsh;

pub struct Shortener {
    id: u64,
    generator: Harsh,
}

impl Shortener {
    pub fn new() -> Shortener {
        let harsh = Harsh::default();
        Shortener {
            id: 0,
            generator: harsh,
        }
    }

    pub fn next_id(&mut self) -> String {
        let hashed = self.generator.encode(&[self.id]);
        self.id += 1;
        hashed
    }
}
