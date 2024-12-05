pub fn pattern15(n: u32) {
    for i in (1..=n).rev() {
        let mut el: char = 'A';
        for j in 1..=i {
            print!("{el}");
            el = ((el as u8) + 1) as char;
        }
        println!();
    }
}
