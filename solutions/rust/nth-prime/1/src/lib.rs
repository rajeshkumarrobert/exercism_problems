pub fn nth(n: u32) -> u32 {
    let mut count = 0;
    let mut number = 2;
    while count < n {
        number += 1;
        if is_prime(number) {
            count += 1;
        }
    }
    number as u32
}

pub fn is_prime(num: u64) -> bool {
    if num <= 1 {
        return false;
    }
    let limit = (num as f64).sqrt() as u64;
    for i in 2..=limit {
        if num % i == 0 {
            return false;
        }
    }
    true
}
