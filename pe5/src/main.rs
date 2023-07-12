fn main() {
    let mut n = 2520;
    let def = 2520;
    let mut z = 1;
    'outer: while true{
        n = def * z;
        z += 1;
        //println!("{}", n);
        let mut passed = 0;
        for x in 1..20 {
            
            if n % x != 0 {
                break;
            }else{
                passed += 1;
            }
            if passed > 18 {
                println!("{} has been found", n);
                break 'outer;
            }
        }
    }
}
