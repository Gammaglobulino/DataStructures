use std::ops::{Add,AddAssign};

#[derive(Debug)]

struct Complex<T>{
    re:T,
    im:T
}

impl<T> Complex<T> {
    fn new(re:T,im:T) ->Complex<T>{
        Complex::<T>{re,im}
    }
}

impl<T> Add for Complex<T>
    where T:Add<Output=T>{
    type Output=Complex<T>;
    fn add(self,rhs:Self) -> Self::Output{
        Complex{
            re:self.re+rhs.re,
            im:self.im+rhs.im
        }
    }
}

impl <T> AddAssign for Complex<T>
    where T:AddAssign<T>{
    fn add_assign(&mut self,rhs:Self){
        self.re+=rhs.re;
        self.im+=rhs.im;
    }
}
pub fn operator_overloading(){
    let mut a=Complex::new(1,2);
    let mut b=Complex::new(1,2);
    println!("a = {:?}",a);
    println!("b = {:?}",b);
    a+=b;
    println!("a+=b -> {:?}",a);
}


