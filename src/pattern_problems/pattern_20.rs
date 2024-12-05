pub fn pattern20 (n: u32) {
    for i in 1..=(2*n -1) {
        if i <= n {
            for j in 1..=i {
                print!("*");
            }
            for j in 1..=(n - i) {
                print!("  ");
            }
            for j in (1..=i).rev() {
                print!("*");
            }
        }
        else {
            for j in (1..=(2*n-i)).rev() {
                print!("*");
            }
            for j in (1..=(i - n)).rev() {
                print!("  ");
            }
            for j in (1..=(2*n-i)).rev() {
                print!("*");
            }
        }
        println!();
    }
}