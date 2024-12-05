pub fn pattern1 (n: u32) {
    for _i in 1..=n {
        for _j in 1..=n {
            print!("*");
        }
        println!();
    }
}