fn main() {
    let mut sum_of_squares: i32 = 0;
    let mut square_of_sum: i32 = 0;
    for n in 0i32..101 {
        sum_of_squares += n.pow(2);
        square_of_sum += n;
    }
    square_of_sum = square_of_sum.pow(2);
    println!("Sum of squares: {sum_of_squares}");
    //let sq = sum.pow(2);
    println!("Square of sum: {square_of_sum}");
    println!("Difference between sums: {}", square_of_sum - sum_of_squares);
}
