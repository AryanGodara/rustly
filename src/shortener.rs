use harsh::{Harsh, HarshBuilder};

pub struct Shortener {
    id: u64,
    generator: Harsh,
}

impl Shortener {
    pub fn new() -> Shortener {
        let harsh = HarshBuilder::new().length(6).build().unwrap();
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