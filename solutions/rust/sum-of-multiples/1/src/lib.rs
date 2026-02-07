use std::collections::HashSet;
pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut values =Vec::new();
    for x in factors{
        values.push(multiples_of(*x, limit));
    }
    let mut seen = HashSet::new();
    let result: Vec<u32> = values
        .into_iter()
        .flatten()
        .filter(|&x| seen.insert(x))
        .collect();
    result.iter().sum()
}

fn multiples_of(n: u32, limit: u32) -> Vec<u32> {
    if n == 0 {
        return Vec::new();
    }
    (n..limit).step_by(n as usize).collect()
}