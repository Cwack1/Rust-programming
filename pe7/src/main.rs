//Slow solution: 3s

fn main() {
    let mut target: u64 = 0;
    let mut counter = 0;
    let mut n = 0;
    while counter < 10001 {
        if is_prime(n){
            counter +=1;
            target = n;
        }
        n += 1;
    }

    println!("{}\nNumber: {}", target, counter);
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