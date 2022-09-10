fn is_prime(n:i32, v:&Vec<i32>) -> bool{
    for &i in v{
        if n%i==0{
            return false;
        }
    }
    return true;
}

pub fn ans(){
    let mut count:i32 = 1;
    let mut prime:Vec<i32> = vec![2];
    let mut temp = 2;
    while count < 10001{
        if is_prime(temp,&prime){
            prime.push(temp);
            count += 1;
        }
        temp += 1;
    }
    println!("{}",prime.last().unwrap());
    // println!("{},{:?}",count,prime);
}