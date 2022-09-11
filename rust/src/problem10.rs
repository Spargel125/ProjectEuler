pub fn ans(){
    let num:i64 = 2000000;
    let mut primelist:Vec<i64> = (0..num).collect();

    let num_root:i64 = (num as f64).powf(0.5) as i64;
    
    // println!("{:?}",primelist);
    for n in 2usize..(num_root+1) as usize{
        if primelist[n]==0{{
            continue;
        }}
        for i in n+1..primelist.len(){
            if primelist[i]%primelist[n]==0{
                primelist[i]=0;
            }
        }
    }
    let mut sum = 0;
    for &i in &primelist{
        sum += i;
    }
    println!("{}",sum-1);
}