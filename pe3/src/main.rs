// THis is a bad solution!

fn main() {
    let mut sum: u64 = 600851475143;
    //while sum > 1 {
        for n in (2..10000).rev() {
            println!("{}", n);
            if sum % n == 0 && is_prime(n){
                println!("Sum: {sum}");
                println!("Divided by: {n}");
                break;
            }
        }
    //}
}

fn is_prime(x: u64) -> bool {
    if x <= 1 {
        return false;
    }
    for a in 2..x {
        if x % a == 0 {
            return false;
        }
    }
    true
}
