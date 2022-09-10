pub fn ans(){
    let mut numvec:Vec<i32>=(1..21).collect();
    // println!("{:?}",numvec);
    for i in 0..20{
        for j in i+1..20{
            if numvec[j]%numvec[i]==0{
                numvec[j] /= numvec[i];
            }
        }
    }

    let mut ans = 1;
    for i in 0..20{
        ans *= numvec[i];
    }
    println!("{}",ans);
}