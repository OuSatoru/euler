pub fn solve() {
    let a = fib_seq().filter(|&x| x%2 == 0).take_while(|&x| x < 4000000u32).fold(0, |s, x| s + x);
    println!("002, {}", a);
}

struct Fibs {
    a: u32,
    b: u32,
}

impl Iterator for Fibs {
    type Item = u32;
    fn next(&mut self) -> Option<u32> {
        let result = self.a + self.b;
        self.a = self.b;
        self.b = result;
        Some(result)
    }
}

fn fib_seq() -> Fibs {
    Fibs{a: 0, b: 1}
}
