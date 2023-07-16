fn main() {
'outer:    for a in 1u32..500 {
        for b in 1u32..500 {
            for c in 1u32..500 {
                if a + b + c == 1000 {
                    if a.pow(2) + b.pow(2) == c.pow(2) {
                        println!("A: {:?} + B: {:?} = C: {:?}", a, b, c);
                        println!("Product: {:?}", a * b * c);
                        break 'outer;
                    }
                }
            }
        }
    }
}
