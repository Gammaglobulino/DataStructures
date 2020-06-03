struct Point{
    x:f64,
    y:f64
}

fn origin()->Point{
    Point{x:0.0,y:0.0}
}


fn main() {

    let p1=origin();
    let p2=Box::new(origin());
    let mut p3=*p2;

    println!("stack origin point x={},y={}",p1.x,p1.y);
    println!("back to stack from heap of point x={},y={}",p3.x,p3.y);
    println!("we are at: {}",if p1.y==0.0 && p1.x==0.0{"origin"}else{""});
    p3.x+=1.0;
    println!("we are at: {}",if p3.x>0.0{"moved to 1"}else{"origin"});

    //while

    while p3.x<10.0{
        p3.x+=1.0;
        println!("{}",p3.x);
    }
    loop {
        p3.x-=1.0;
        println!("{}",p3.x);
        if p3.x == 3.0{break;}
    }
    for x in 1..5{
        p3.x+= x as f64;
        println!("{}",p3.x)
    }

}
