pub fn pattern12 (n: u32) {
    for i in 1..=n {
        for j in 1..=i {
            print!("{j}");
        }
        for j in (1..=(2*n - 2*i)).rev() {
            print!(" ");
        }
        for j in (1..=i).rev() {
            print!("{j}");
        }
        println!();
    }
}