pub fn ans(){
    for a in 1i32..1000i32{
        for b in a..1000{
            if a.pow(2)+b.pow(2)==(1000-a-b).pow(2){
                println!("a={},b={},c={}",a,b,1000-a-b);
                println!("ans = {}",a*b*(1000-a-b));
            }
        }
    }
}