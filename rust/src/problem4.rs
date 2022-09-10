pub fn ans(){
    let mut s :i64= 0;
    let mut max = 0;
    for i in 100..999{
        for j in 100..999{
            s = i*j;
            if s.to_string()==s.to_string().chars().rev().collect::<String>(){
                if s > max{
                    max = s;
                }
            }
        }
    }
    println!("{}",max);

}