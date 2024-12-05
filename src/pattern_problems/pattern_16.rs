pub fn pattern16(n: u32) {
    let mut el: char = 'A';
    for i in 1..=n {
        for j in 1..=i {
            print!("{el}");
        }
        el = ((el as u8) + 1) as char;
        println!();
    }
}
