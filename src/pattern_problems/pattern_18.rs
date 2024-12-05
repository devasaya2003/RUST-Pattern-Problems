pub fn pattern18 (n: u32) {
    for i in 1..=n {
        let mut el: char = 'A';
        el = ((el as u8) + ((n as u8) - (i as u8))) as char;
        for j in 1..=i {
            print!("{el} ");
            el = ((el as u8) + 1) as char;
        }
        println!();
    }
}