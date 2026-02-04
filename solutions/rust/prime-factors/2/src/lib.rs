pub fn factors(mut n: u64) -> Vec<u64> {
    let mut res = Vec::new();
    let mut divisor = 2;
    while divisor * divisor <= n {
        while n.is_multiple_of(divisor){
            res.push(divisor);
            n /= divisor;
        }
        divisor += 1;
    }
    if n > 1 {
        res.push(n);
    }
    res
}