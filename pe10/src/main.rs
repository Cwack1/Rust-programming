fn main() {
    let mut sum = 0;
    for n in 0u64..2_000_000 {
        if is_prime(n){
            //println!("is prime: {:?}", n);
            sum += n;
        }
    }
    println!("{sum}");
}

fn is_prime(x: u64) -> bool {
    let limit = (x as f64).sqrt() as u64;
    if x <= 1 {
        return false;
    }
    for a in 2..=limit {
        if x % a == 0 {
            return false;
        }
    }
    true
}