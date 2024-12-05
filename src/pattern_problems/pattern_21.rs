pub fn pattern21 (n: u32) {
    for i in 1..=n {
        for j in 1..=n {
            if j == 1 || j == n || i == 1 || i == n {
                print!("*");
            } else {
                print!(" ");
            }
        }
        println!();
    }
}