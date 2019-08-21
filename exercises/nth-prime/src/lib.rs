use std::f64;

pub fn nth(n: u32) -> u32 {
    let mut current_prime = 2;
    let mut num = 2;
    for _ in 0..=n {
        loop {
            if is_prime(num) {
                current_prime = num;
                num += 1;
                break;
            }
            num += 1;
        }
    }

    current_prime
}

fn is_prime(n: u32) -> bool {
    let bound = f64::from(n).sqrt() as u32;
    for i in 2..=bound {
        if n % i == 0 {
            return false;
        }
    }
    true 
}