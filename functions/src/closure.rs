
fn say_hi(){
    println!("Hi there!");
}


pub fn closure(){
    let sh=say_hi;
    sh();
    //anon function
    let plus_one = |x:i32|->i32{x+1};
    let a =6;
    println!("{}+1={}",a,plus_one(a));

    let mut two=2;
    {
        let plus_two = |x| {
            let mut z = x;
            z += two; //<--closure
            z
        };
        println!("3+2={}",plus_two(3));
    }//<--closure released

    let borrow_of_two=&mut two;
    print!("{}",borrow_of_two);

    //pass by mutable reference &mut

    let increase_by_three=|x:&mut i32|{ *x+=3};
    let mut x=1;
    increase_by_three(&mut x);
    println!("increassing x=1 by 3 ={}",x);

}