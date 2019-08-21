pub fn factors(n: u64) -> Vec<u64> {
    let mut result: Vec<u64> = vec![];
    let mut num = n;
    let mut i = 2;

    while i * i <= n {
        while num % i == 0 {
            result.push(i);
            num /= i;
        }
        i += 1;
    }
    if num != 1 {
        result.push(num);
    }

    result
}
