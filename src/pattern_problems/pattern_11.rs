pub fn pattern11 (n: u32) {
    for i in 1..=n {
        for j in 1..=i {
            if (i+j) % 2 == 0 {
                print!("1");
            } else {
                print!("0");
            }
        }
        println!();
    }
}