pub fn pattern6 (n: u32) {
    for i in (1..=n).rev() {
        for j in 1..=i {
            print!("{j}");
        }
        println!();
    }
}