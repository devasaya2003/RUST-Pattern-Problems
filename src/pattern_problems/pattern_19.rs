pub fn pattern19 (n: u32) {
    for i in 1..=n {
        for j in (1..=(n - i + 1)).rev() {
            print!("*");
        }
        for j in 1..=(i-1) {
            print!("  ");
        }
        for j in 1..=(n - i + 1) {
            print!("*");
        }
        
        println!();
    }
    for i in (1..=n).rev() {
        for j in (1..=(n - i + 1)).rev() {
            print!("*");
        }
        for j in 1..=(i-1) {
            print!("  ");
        }
        for j in 1..=(n - i + 1) {
            print!("*");
        }
        
        println!();
    }
}