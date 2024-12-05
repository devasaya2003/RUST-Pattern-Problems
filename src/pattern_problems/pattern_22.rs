use std::cmp::min;

pub fn pattern22 (n: u32) {
    for i in 1..=(2*n - 1) {
        for j in 1..=(2*n - 1) {
            let top = i - 1;
            let bottom = j - 1;
            let left = 2*n- 1 - i;
            let right = 2*n - 1 - j;
            print!("{} ", (n - min(min(top, bottom), min(left, right))));
        }
        println!();
    }
}