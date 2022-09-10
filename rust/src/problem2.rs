pub fn ans() {
    let mut a1: i32 = 1;
    let mut a2: i32 = 2;
    let mut s: i32 = 0;
    let mut sum: i32 = a2;
    while s < 4000000 {
        s = a1 + a2;
        if s % 2 == 0 {
            sum += s;
        }
        a1 = a2;
        a2 = s;
    }
    println!("{}", sum);
}
