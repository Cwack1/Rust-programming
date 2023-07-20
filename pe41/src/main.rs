fn main() {
    for i in (1234567..7654322).rev() {
        
        if one_nine(i.to_string()){
            if is_pandigital(i.to_string()){
                //println!("{:?} is pandigital", i);
                if is_prime(i){
                    println!("{:?} is prime and pandigital 1-9", i);
                    break;
                }
            }
        }
        
    }
}

fn is_pandigital(s: String) -> bool {
    for a in 0usize..s.len(){
        for b in a+1..s.len(){
            //println!("{:?} == {:?}", a, b);
            if s.chars().nth(a) == s.chars().nth(b) {
                //println!("{:?} == {:?}", a, b);
                return false;
            }
        }
    }

    true
}

fn one_nine(s: String) -> bool {
    for c in 1..8 {
        if !s.contains(&c.to_string()){
            return false;
        }
    }
    true
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