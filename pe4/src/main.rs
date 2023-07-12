fn main() {
    let mut biggest = 0;
    'outer: for x in (500..1000).rev() {
        for y in (500..1000).rev(){
            let s = (x*y).to_string();
            if reverse_is_palindrome(&s) {
                //println!("Palindrome found: {}", s);
                //println!("x: {}\ny: {}\nSum: {}", x, y, x*y);
                if x * y > biggest {
                    biggest = x * y;
                }
            }
        }
    }
    println!("Largest palindrome 3x3 digits: {}", biggest);
}

fn reverse_is_palindrome(b: &str) -> bool {
    let c = reverse(b);
    if c == b {
        //println!("b: {}\nc: {}", b, c);
        return true;
    }
    false
}

fn reverse(s: &str) -> String {
    s.chars().rev().collect()
}
