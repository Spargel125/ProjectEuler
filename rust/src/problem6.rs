pub fn ans() {
    let num: i64 = 100;
    let mut sum1: i64 = 0;
    let mut sum2: i64 = 0;
    for n in 0..num + 1 {
        sum1 += n.pow(2u32);
        sum2 += n;
    }
    println!("{}", sum1 - sum2.pow(2u32));
}
