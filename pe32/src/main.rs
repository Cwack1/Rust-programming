fn main() {
    let mut sum = 0;
    let mut vec = Vec::new();
    //A 2 digit
    //B 3 digit
    //C 5 digit
    for a in 10..100 {
        if !a.to_string().contains("0"){
            if is_pandigital(a.to_string()){
                for b in 100..1000{
                    if !b.to_string().contains("0") && a*b < 100_000 && !(a*b).to_string().contains("0"){
                        if is_pandigital(merge_strings(a, b)) {
                            let c = a * b;
                            //if !c.to_string().contains("0"){
                                if is_pandigital(merge_all_strings(a, b, c)){
                                    if one_nine(merge_all_strings(a, b, c)){
                                        
                                        if !vec.contains(&c){
                                            println!("{:?} * {:?} = {:?}", a, b, c);
                                            vec.push(c);
                                            sum += c;
                                        }
                                    }
                                }
                            //}

                        }
                    }
                }
            }
        }

        
    }
    //A 1 digit
    //B 4 digit
    //C 5 digit
    for a in 1..10 {
        if !a.to_string().contains("0"){
            if is_pandigital(a.to_string()){
                for b in 1000..9877{
                    if !b.to_string().contains("0") && a*b < 100_000 && !(a*b).to_string().contains("0"){
                        if is_pandigital(merge_strings(a, b)) {
                            let c = a * b;
                            //if !c.to_string().contains("0"){
                                if is_pandigital(merge_all_strings(a, b, c)){
                                    if one_nine(merge_all_strings(a, b, c)){
                                        
                                        if !vec.contains(&c){
                                            println!("{:?} * {:?} = {:?}", a, b, c);
                                            vec.push(c);
                                            sum += c;
                                        }
                                    }
                                }
                            //}

                        }
                    }
                }
            }
        }

        
    }
/* 
    let n = 15234;
    let pan = is_pandigital(n.to_string());
    if pan == true {
        println!("{:?} is pandigital 1 - 9", n);
    }else {
        println!("{:?} is NOT pandigital  - 9", n);
    }

    let a = 39;
    let b = 186;
    let tst = merge_strings(a, b, a * b);
    if is_pandigital(tst.clone()) == true {
        println!("{:?} is pandigital 1 - 9 ", tst);
    }else {
        println!("{:?} is NOT pandigital 1 - 9", tst);
    }
     */
    println!("Sum: {:?}", sum);
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


fn merge_strings(a: i32, b: i32) -> String {
    a.to_string() + &b.to_string()
}

fn merge_all_strings(a: i32, b: i32, c: i32) -> String {
    a.to_string() + &b.to_string() + &c.to_string()
}
