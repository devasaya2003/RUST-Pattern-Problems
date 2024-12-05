pub fn pattern3 (n: u32) {
    for i in 1..=n {
        for j in 1..=i {
            print!("{j}");
        }
        println!();
    }
}