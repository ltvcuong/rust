pub fn is_armstrong_number(num: u32) -> bool {
    let mut digits: Vec<u32> = vec![];
    let mut n = num;
    while n != 0 {
        digits.push(n % 10);
        n /= 10;
    }

    let sum: u32 = digits.iter().map(|x| x.pow(digits.len() as u32)).sum();
    sum == num
}
