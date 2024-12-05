pub fn pattern4 (n: u32) {
    for i in 1..=n {
        for j in 1..=i {
            print!("{i}");
        }
        println!();
    }
}