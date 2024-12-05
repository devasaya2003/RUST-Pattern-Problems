pub fn pattern17 (n: u32) {
    for i in 1..=n {
        let mut el: char = 'A';
        for j in 1..=(n-i) {
            print!(" ");
        }
        for j in 1..=(2*i - 1) {
            print!("{el}");
            if j > (i-1) {
                el = ((el as u8) - 1) as char;
            } else {
                el = ((el as u8) + 1) as char;
            }
        }
        println!();
    }
}