fn is_even(val:u32)->bool{
    val % 2 ==0
}

fn greater_than(limit:u32)
->impl Fn(u32)->bool
{
    move |y| y > limit
}

pub fn hof(){

    let limit=500;
    let mut sum =0;

    let above_limit=greater_than(limit);

    for i in 0..{
        let isq=i*i;
        if above_limit(isq){
            break;
        }else if is_even(isq){
            sum+=isq;
        }
    }
    println!("sum of first 500 even square={}",sum);

    // same result using collections

    let sum2=(0..)
        .map(|x|x*x)
        .take_while(|&x|x<limit)
        .filter(|x|is_even(*x))
        .fold(0,|sum,x|sum+x);

    println!("sum of first 500 even square={}",sum2);
}