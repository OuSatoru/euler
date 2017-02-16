use fib;

pub fn solve() {
    let a = fib::fib_seq().filter(|&x| x%2 == 0).take_while(|&x| x < 10u32).fold(0, |s, x| s + x);
    println!("{:?}", a);
}
