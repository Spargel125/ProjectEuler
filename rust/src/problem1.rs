pub fn main() {
    let num: i32= 1000;
    let mut sum: i32 = 0;
    for i in 1..num {
        if i%3==0 || i%5==0{
            sum += i;
        }
    }
    println!("{}",sum);

}
