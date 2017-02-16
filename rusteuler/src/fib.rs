pub struct Fibs {
    a: u32,
    b: u32,
}

impl Iterator for Fibs {
    fn next(&mut self) -> Option<u32> {
        let mut result = self.a + self.b;
        self.a = self.b;
        self.b = result;
        Some(result)
    }
}

pub fn fib_seq() -> Fibs {
    Fibs(a: 0, b: 0);
}
