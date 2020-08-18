use std::fmt::Debug;

#[derive(Debug)]
struct Circle{
    radius:f64,
}
#[derive(Debug)]
struct Square{
    side:f64,
}

trait Shape{
    fn area(&self)->f64;
}

impl Shape for Square{
    fn area(&self)->f64{
        self.side*self.side
    }
}
impl Shape for Circle{
    fn area(&self)->f64{
        self.radius*self.radius *std::f64::consts::PI
    }
}

//fn print_shape_info(shape:impl Shape+Debug){
//fn print_shape_info<T:Shape+Debug>(shape:T){
fn print_shape_info<T>(shape:T)
where T:Shape+Debug
{
    println!("The area is {}",shape.area());
    println!("{:?}",shape);
}

pub fn traits_as_parameter(){
    let c=Circle{radius:2.0};
    print_shape_info(c);

}