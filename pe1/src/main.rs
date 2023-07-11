
fn main() {
    let mut three = 3;
    let mut five = 5;
    let mut sum = 0;

    while three < 1000 {
        //println!("{three}");
        sum += three;
        three += 3;
    }

    while five < 1000 {
        if five % 3 != 0 {
            //println!("{five}");
            sum += five;
        }
        five += 5;
    }
    println!("{sum}");
}
