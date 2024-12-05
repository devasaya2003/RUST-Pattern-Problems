pub fn pattern10 (n: u32) {
    for i in 1..=(2*n - 1) {
        if i <= n {
            for j in 1..=i {
                print!("*");
            }
            println!();
        } else {
            for j in 1..=(2*n - i) {
                print!("*");
            }
            println!();
        }
    }
}