pub fn pattern7 (n: u32) {
    for i in 1..=n {
        for j in 1..=(n-i) {
            print!(" ");
        }
        for j in 1..=(2*i-1) {
            print!("*");
        }
        println!();
    }
}