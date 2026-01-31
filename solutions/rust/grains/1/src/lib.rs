pub fn square(s: u32) -> u64 {
    if s < 65{
        2_u64.pow(s-1)
    }else {
        panic!()
    }  
}

pub fn total() -> u64 {
    (1..=64).map(|x| square(x)).sum::<u64>()
}
