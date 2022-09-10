pub fn ans(){
    let num:f64 = 600851475143.0;
    let num_root: i64 = num.powf(1.0/2.0) as i64;
    let mut primenum: Vec<i64> = vec![2];
    // let mut numvec:Vec<i32>=(0..num_root).collect();
    // println!("{:?}",numvec.len());

    let mut num2:i64 = num as i64;
    let num : i64 = num as i64;
    print!("{}",num);

    for n in 2..num_root{
        if num2 % n == 0{
            primenum.push(n);
            while num2 % n ==0{
                num2 /= n;
            }    
        }
        if num2 == 1{
            print!("{:?}",primenum);
            println!("{}",n);
            break;
        }
    }
    println!("{:?}",primenum);

    // println!("{}",ans);
}