mod fakedata;
use std::collections::HashMap;
use std::time::Instant;

fn fibonacci(mut memo: HashMap<usize, u32>, num: u32) -> u32 {
    if num < 2 {
        num
    } else if memo.contains_key(num as &usize) {
        memo.get(num).unwrap()
    } else {
        let result = fibonacci(memo, num - 1) + fibonacci(memo, num - 2);

        memo.insert(num, result);

        result
    }
}

fn main() {
    let mut memo: HashMap<usize, u32> = HashMap::new();

    let now = Instant::now();

    let sum: u32 = fibonacci(memo, 50);

    let elapsed = now.elapsed();

    println!("{} - {:.2?}", sum, elapsed);
}
