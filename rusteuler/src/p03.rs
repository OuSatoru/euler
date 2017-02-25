pub fn solve() {
    let mut array: Vec<i64> = Vec::new();
    let mut i: i64 = 2;
    let mut max: i64 = 600851475143;
    while i*i < max {
        while max%i == 0 {
            array.push(i);
            max /= i;
        }
        i += 1;
    }
    if max > 1 {
        array.push(max)
    }
    let result = match array.iter().max() {
        Some(&x) => x,
        None => 0i64,
    };
    println!("003, {}", result);
}
