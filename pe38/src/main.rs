fn main() {
    let mut ans_str = String::from("");
    let mut large = String::from("");
    //A 1 Digit
    //B 1 digit
    //C 1-2 digits
    for i in 9..50_000 {
        ans_str = "".to_string();
        for n in 1..9 {
            // n > 1
            ans_str += &(i * n).to_string();
            if !is_pandigital(ans_str.clone()){
                break;
            }else{
                if one_nine(ans_str.clone()){
                    println!("Answer: {:?} x {:?} = {:?}", i, n, ans_str);
                    if ans_str > large {
                        large = ans_str.clone();
                    }
                }
            }
        }
        //ans_str += &(i * 9).to_string();
        //println!("{:?}", runCheck(ans_str));    
    }
    println!("largest: {:?}", large);
    //println!("{:?}", runCheck(ans_str));
}

fn runCheck(s: String) -> String {
    if !s.contains("0"){
        if is_pandigital(s.clone()) {
            if one_nine(s.clone()) {
                return s + " is pandigital 1-9";
            }else{
                return s + " is pandigital, but NOT 1-9";
            }
        }else{
            return s + " is NOT pandigital";
        }
    }
    "Error".to_string()
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
    for c in 1..10 {
        if !s.contains(&c.to_string()){
            return false;
        }
    }
    true
}