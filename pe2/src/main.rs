fn main() {
    let mut num = 1;
    let mut two = 2;
    let mut sum = 0;

    while num < 4000000 || two < 4000000 {
        if num % 2 == 0 {
            sum += num;
        }
        if two % 2 == 0 {
            sum += two;
        }
        //let mut tmp = num;
        num += two;
        two += num;
        //println!("num: {num}");
        //println!("two: {two}");
    }
    println!("Sum: {sum}");
}
