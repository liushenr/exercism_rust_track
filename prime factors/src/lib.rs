pub fn factors(mut n: u64) -> Vec<u64> {
    let mut result = Vec::new();
        
    while n > 1 {
        let i = (2..=n).find(|x| n % x ==0).unwrap();
        result.push(i);
        n /= i;
    }
    result
}
