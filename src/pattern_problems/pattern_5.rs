pub fn pattern5 (n: u32) {
    for i in (1..=n).rev() {
        for j in 1..=i {
            print!("*");
        }
        println!();
    }
}